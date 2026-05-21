# View Configuration DSL Specification

The view configuration DSL is a text-based language for configuring
database views. Directives are separated by semicolons or newlines.
Property names must be double-quoted. Keywords are case-insensitive.

## Directives

### FILTER
Filter rows based on property values.

Syntax:
  FILTER "Property" operator value

Operators:
  = != > < >= <=          Comparison operators
  CONTAINS                Text contains substring
  STARTS WITH             Text starts with prefix
  ENDS WITH               Text ends with suffix
  IS EMPTY                Property has no value
  IS NOT EMPTY            Property has a value
  IN ("val1", "val2")     Value is in set

Compound filters:
  AND                     Both conditions must be true
  OR                      Either condition can be true
  ( )                     Group conditions

Multiple FILTER directives are ANDed together.

Examples:
  FILTER "Status" = "In Progress"
  FILTER "Priority" > 3
  FILTER "Name" CONTAINS "report"
  FILTER "Assignee" IS NOT EMPTY
  FILTER "Status" IN ("Done", "Archived")
  FILTER ("Status" = "Done" OR "Status" = "Archived") AND "Priority" > 3
  FILTER "Status" = "In Progress"; FILTER "Assignee" IS NOT EMPTY

### SORT BY
Sort rows by one or more properties.

Syntax:
  SORT BY "Property" [ASC|DESC] [, "Property" [ASC|DESC] ...]

Default direction is ASC (ascending).

Examples:
  SORT BY "Due Date" ASC
  SORT BY "Priority" DESC, "Name" ASC
  SORT BY "Created Time"

### GROUP BY
Group rows by a property value. Required for board views.

Syntax:
  GROUP BY "Property"

Examples:
  GROUP BY "Status"
  GROUP BY "Priority"

### CALENDAR BY
Set the date property for calendar views. Required for calendar type.

Syntax:
  CALENDAR BY "Property"

The property must be a date, created_time, last_edited_time,
or date formula property.

Examples:
  CALENDAR BY "Due Date"

### TIMELINE BY
Set the date range for timeline views. Required for timeline type.

Syntax:
  TIMELINE BY "Start Property" [TO "End Property"]

Both properties must be date types.

Examples:
  TIMELINE BY "Start Date" TO "End Date"
  TIMELINE BY "Due Date"

### SHOW
Set which properties are visible (in order).

Syntax:
  SHOW "Property1", "Property2", ...

Examples:
  SHOW "Name", "Status", "Assignee"

### HIDE
Hide specific properties.

Syntax:
  HIDE "Property1", "Property2", ...

Examples:
  HIDE "Created Time", "Last Edited"

### COVER
Set cover image property for gallery/board views.

Syntax:
  COVER "Property" [SIZE small|medium|large] [ASPECT cover|contain]

Examples:
  COVER "Files & media"
  COVER "Image" SIZE medium ASPECT cover

### WRAP CELLS
Enable or disable cell wrapping in table views.

Syntax:
  WRAP CELLS true|false

### FREEZE COLUMNS
Set number of frozen columns in table views.

Syntax:
  FREEZE COLUMNS <number>

Examples:
  FREEZE COLUMNS 1
  FREEZE COLUMNS 2

### MAP BY
Set the location property for map views. Required for map type.

Syntax:
  MAP BY "Property"

The property should contain location/place data.

Examples:
  MAP BY "Location"

### CHART
Configure a chart view. Required for chart type.

Syntax:
  CHART column|bar|line|donut|number
    [AGGREGATE count|sum|average|min|max|... [ON "Property"]]
    [COLOR gray|blue|green|purple|orange|red|auto|colorful]
    [HEIGHT small|medium|large|extra_large]
    [SORT x_ascending|x_descending|y_ascending|y_descending]
    [STACK BY "Property"]
    [CAPTION "text"]

Chart directives can appear on the same line. AGGREGATE defaults to
count if omitted. Use GROUP BY to set the x-axis grouping property.

Examples:
  CHART column
  GROUP BY "Status"; CHART column AGGREGATE count
  GROUP BY "Month"; CHART line AGGREGATE sum ON "Revenue" COLOR blue
  GROUP BY "Category"; CHART bar AGGREGATE average ON "Score" STACK BY "Region"
  CHART number AGGREGATE sum ON "Total"
  CHART donut; GROUP BY "Status"

### FORM
Configure a form view.

Syntax:
  FORM CLOSE          Close form to new submissions
  FORM OPEN           Open form to submissions
  FORM ANONYMOUS true|false    Toggle anonymous submissions
  FORM PERMISSIONS none|comment_only|reader|read_and_write|editor

Examples:
  FORM CLOSE
  FORM ANONYMOUS true
  FORM PERMISSIONS reader

## CLEAR Directives (update_view only)
Remove existing settings.

  CLEAR FILTER       Remove all filters
  CLEAR SORT         Remove all sorts
  CLEAR GROUP BY     Remove grouping

## Combining Directives
Separate multiple directives with semicolons or newlines:

  GROUP BY "Status"; SORT BY "Due Date" ASC; FILTER "Priority" > 3

  GROUP BY "Status"
  SORT BY "Due Date" ASC
  FILTER "Priority" > 3