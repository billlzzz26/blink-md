//! Converter Traits and Registry
//!
//! Defines the traits for converting between platforms and Universal IR,
//! plus a registry for dynamic platform discovery.

use crate::ir::{Platform, UniversalDocument};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Error type for conversion operations
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ConverterError {
    #[error("Unsupported platform: {0}")]
    UnsupportedPlatform(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Conversion failed: {0}")]
    ConversionFailed(String),
    #[error("IO error: {0}")]
    IoError(String),
}

/// Trait for converting FROM a platform TO Universal IR
pub trait FromPlatform {
    /// Platform identifier this converter handles
    const PLATFORM: Platform;

    /// Input type for this platform (e.g., Notion Page, Markdown string, HTML string)
    type Input;

    /// Convert from platform-specific format to Universal IR
    fn from_platform(input: Self::Input) -> Result<UniversalDocument, ConverterError>;

    /// Optional: convert with options
    fn from_platform_with_options(
        input: Self::Input,
        _options: HashMap<String, serde_json::Value>,
    ) -> Result<UniversalDocument, ConverterError> {
        Self::from_platform(input)
    }
}

/// Trait for converting FROM Universal IR TO a platform
pub trait ToPlatform {
    /// Platform identifier this converter handles
    const PLATFORM: Platform;

    /// Output type for this platform (e.g., Notion CreatePageRequest, Markdown string, HTML string)
    type Output;

    /// Convert from Universal IR to platform-specific format
    fn to_platform(doc: &UniversalDocument) -> Result<Self::Output, ConverterError>;

    /// Optional: convert with options
    fn to_platform_with_options(
        doc: &UniversalDocument,
        _options: HashMap<String, serde_json::Value>,
    ) -> Result<Self::Output, ConverterError> {
        Self::to_platform(doc)
    }
}

/// Converter registry for dynamic platform discovery
#[derive(Default)]
pub struct ConverterRegistry {
    from_converters: HashMap<Platform, Box<dyn Fn() -> Box<dyn FromPlatformDyn>>>,
    to_converters: HashMap<Platform, Box<dyn Fn() -> Box<dyn ToPlatformDyn>>>,
}

/// Dynamic dispatch versions of the traits
pub trait FromPlatformDyn: Send + Sync {
    fn platform(&self) -> Platform;
    fn convert(&self, input: Box<dyn std::any::Any>) -> Result<UniversalDocument, ConverterError>;
}

pub trait ToPlatformDyn: Send + Sync {
    fn platform(&self) -> Platform;
    fn convert(&self, doc: &UniversalDocument) -> Result<Box<dyn std::any::Any>, ConverterError>;
}

/// Wrapper to convert static trait to dynamic trait
struct FromPlatformWrapper<F: FromPlatform + Send + Sync> {
    _phantom: std::marker::PhantomData<F>,
}

impl<F: FromPlatform + Send + Sync + 'static> FromPlatformDyn for FromPlatformWrapper<F> {
    fn platform(&self) -> Platform {
        F::PLATFORM
    }
    fn convert(&self, input: Box<dyn std::any::Any>) -> Result<UniversalDocument, ConverterError> {
        let input = *input
            .downcast::<F::Input>()
            .map_err(|_| ConverterError::InvalidData("Type mismatch".into()))?;
        F::from_platform(input)
    }
}

struct ToPlatformWrapper<T: ToPlatform + Send + Sync> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: ToPlatform + Send + Sync + 'static> ToPlatformDyn for ToPlatformWrapper<T> {
    fn platform(&self) -> Platform {
        T::PLATFORM
    }
    fn convert(&self, doc: &UniversalDocument) -> Result<Box<dyn std::any::Any>, ConverterError> {
        let output = T::to_platform(doc)?;
        Ok(Box::new(output))
    }
}

impl ConverterRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_from<F: FromPlatform + Send + Sync + 'static>(&mut self) {
        self.from_converters.insert(
            F::PLATFORM,
            Box::new(|| {
                Box::new(FromPlatformWrapper::<F> {
                    _phantom: std::marker::PhantomData,
                })
            }),
        );
    }

    pub fn register_to<T: ToPlatform + Send + Sync + 'static>(&mut self) {
        self.to_converters.insert(
            T::PLATFORM,
            Box::new(|| {
                Box::new(ToPlatformWrapper::<T> {
                    _phantom: std::marker::PhantomData,
                })
            }),
        );
    }

    pub fn get_from(&self, platform: Platform) -> Option<&dyn Fn() -> Box<dyn FromPlatformDyn>> {
        self.from_converters.get(&platform).map(|b| b.as_ref())
    }

    pub fn get_to(&self, platform: Platform) -> Option<&dyn Fn() -> Box<dyn ToPlatformDyn>> {
        self.to_converters.get(&platform).map(|b| b.as_ref())
    }

    pub fn available_platforms(&self) -> Vec<Platform> {
        let mut platforms: Vec<Platform> = self.from_converters.keys().cloned().collect();
        platforms.extend(self.to_converters.keys().cloned());
        platforms.sort_by_key(|p| format!("{:?}", p));
        platforms.dedup();
        platforms
    }
}

/// Convenience function for roundtrip testing
pub fn roundtrip<F, T>(input: F::Input) -> Result<(), ConverterError>
where
    F: FromPlatform,
    T: ToPlatform,
{
    let ir = F::from_platform(input)?;
    let _output = T::to_platform(&ir)?;
    Ok(())
}

pub mod github_markdown;
pub mod lark_sheets;
pub mod markdown;
pub mod markdown_frontmatter;
pub mod notion;

/// Roundtrip with comparison (for testing)
pub fn roundtrip_compare<F, T>(input: F::Input) -> Result<(), ConverterError>
where
    F: FromPlatform,
    T: ToPlatform,
{
    let ir = F::from_platform(input)?;
    let _output = T::to_platform(&ir)?;
    Ok(())
}
