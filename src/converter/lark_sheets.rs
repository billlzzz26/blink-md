//! Lark/Feishu Sheets and CSV Adapter
//!
//! Maps tabular data between Universal IR and Lark/CSV/Google Sheets formats.

use crate::converter::{ConverterError, FromPlatform, ToPlatform};
use crate::ir::{
    table::TableCell, table::TableRow, table::TableRowType, Platform, UniversalBlock,
    UniversalDocument,
};

/// Lark/CSV Adapter
pub struct LarkSheetAdapter;

impl FromPlatform for LarkSheetAdapter {
    const PLATFORM: Platform = Platform::Sheets;
    type Input = String; // CSV data

    fn from_platform(input: Self::Input) -> Result<UniversalDocument, ConverterError> {
        let mut reader = csv::Reader::from_reader(input.as_bytes());
        let mut rows = Vec::new();

        // Header
        let headers = reader
            .headers()
            .map_err(|e| ConverterError::InvalidData(e.to_string()))?
            .clone();
        let header_row = TableRow {
            row_type: TableRowType::Header,
            style: None,
            cells: headers
                .iter()
                .map(|h| TableCell {
                    content: vec![crate::ir::inline::text(h)],
                    colspan: None,
                    rowspan: None,
                    style: None,
                    align: None,
                })
                .collect(),
        };
        rows.push(header_row);

        // Body
        for result in reader.records() {
            let record = result.map_err(|e| ConverterError::InvalidData(e.to_string()))?;
            let body_row = TableRow {
                row_type: TableRowType::Body,
                style: None,
                cells: record
                    .iter()
                    .map(|c| TableCell {
                        content: vec![crate::ir::inline::text(c)],
                        colspan: None,
                        rowspan: None,
                        style: None,
                        align: None,
                    })
                    .collect(),
            };
            rows.push(body_row);
        }

        let table_block = UniversalBlock::Table {
            rows,
            header: None,
            style: None,
        };

        Ok(UniversalDocument {
            metadata: crate::ir::DocumentMetadata::default(),
            blocks: vec![table_block],
            styles: crate::ir::StyleSheet::default(),
        })
    }
}

impl ToPlatform for LarkSheetAdapter {
    const PLATFORM: Platform = Platform::Sheets;
    type Output = String; // CSV representation

    fn to_platform(doc: &UniversalDocument) -> Result<Self::Output, ConverterError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        for block in &doc.blocks {
            if let UniversalBlock::Table { rows, .. } = block {
                for row in rows {
                    let record: Vec<String> = row
                        .cells
                        .iter()
                        .map(|cell| {
                            cell.content
                                .iter()
                                .map(|inline| match inline {
                                    crate::ir::InlineElement::TextRun { content, .. } => {
                                        content.clone()
                                    }
                                    _ => String::new(),
                                })
                                .collect::<Vec<String>>()
                                .join("")
                        })
                        .collect();
                    wtr.write_record(&record)
                        .map_err(|e| ConverterError::ConversionFailed(e.to_string()))?;
                }
            }
        }

        let data = String::from_utf8(wtr.into_inner().unwrap())
            .map_err(|e| ConverterError::ConversionFailed(e.to_string()))?;
        Ok(data)
    }
}
