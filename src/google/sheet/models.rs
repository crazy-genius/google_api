use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use super::enums::*;

/// A type `Spreadsheet` represents a spreadsheet.
#[derive(Serialize, Deserialize)]
pub struct Spreadsheet {
    /// Overall properties of a spreadsheet.
    pub properties: SpreadsheetProperties,
    /// The sheets that are part of a spreadsheet.
    pub sheets: Vec<Sheet>,
    #[serde(rename(serialize = "dataSources", deserialize = "dataSources"))]
    /// A list of external data sources connected with the spreadsheet.
    pub data_sources: Option<Vec<DataSource>>,
    #[serde(rename(serialize = "developerMetadata", deserialize = "developerMetadata"))]
    /// The developer metadata associated with a spreadsheet..
    pub developer_metadata: Option<Vec<DeveloperMetadata>>,
    #[serde(rename(serialize = "dataSourceSchedules", deserialize = "dataSourceSchedules"))]
    //#[readonly]
    /// Output only. A list of data source refresh schedules.
    data_source_schedules: Option<Vec<DataSourceRefreshSchedule>>,
    #[serde(rename(serialize = "spreadsheetUrl", deserialize = "spreadsheetUrl"))]
    //#[readonly]
    /// The url of the spreadsheet. This field is read-only.
    spreadsheet_url: String,
    #[serde(rename(serialize = "namedRanges", deserialize = "namedRanges"))]
    /// The named ranges defined in a spreadsheet.
    pub named_ranges: Option<Vec<NamedRange>>,
    #[serde(rename(serialize = "spreadsheetId", deserialize = "spreadsheetId"))]
    //#[readonly]
    /// The ID of the spreadsheet. This field is read-only.
    spreadsheet_id: String,
}

impl Spreadsheet {
    pub fn spreadsheet_id(&self) -> String {
        self.spreadsheet_id.clone()
    }
    pub fn spreadsheet_url(&self) -> String {
        self.spreadsheet_url.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct SpreadsheetProperties {}

#[derive(Serialize, Deserialize)]
pub struct SheetProperties {}

#[derive(Serialize, Deserialize)]
pub struct ProtectedRange {}

#[derive(Serialize, Deserialize)]
pub struct BasicFilter {}

#[derive(Serialize, Deserialize)]
pub struct EmbeddedChart {}

#[derive(Serialize, Deserialize)]
pub struct ConditionalFormatRule {}

#[derive(Serialize, Deserialize)]
pub struct DimensionGroup {}

#[derive(Serialize, Deserialize)]
pub struct BandedRange {}

#[derive(Serialize, Deserialize)]
pub struct CellData {}

#[derive(Serialize, Deserialize)]
pub struct EmbeddedObjectPosition {}

#[derive(Serialize, Deserialize)]
pub struct TextFormat {}

#[derive(Serialize, Deserialize)]
pub struct FilterCriteria {}

#[derive(Serialize, Deserialize)]
pub struct FilterSpec {}

#[derive(Serialize, Deserialize)]
pub struct SortSpec {}

#[derive(Serialize, Deserialize)]
pub struct BigQueryQuerySpec {}

/// A color value.
#[derive(Serialize, Deserialize)]
pub struct ColorStyle {
    /// Theme color.
    #[serde(rename(serialize = "themeColor", deserialize = "themeColor"))]
    theme_color: ThemeColor,

    /// RGB color.
    #[serde(rename(serialize = "rgbColor", deserialize = "rgbColor"))]
    rgb_color: Color,
}

/// Represents a color in the RGBA color space.
/// This representation is designed for simplicity of conversion to/from color representations in various languages over compactness.
/// For example, the fields of this representation can be trivially provided to the constructor of `java.awt.Color` in Java; it can also be trivially provided to UIColor's `+colorWithRed:green:blue:alpha` method in iOS; and, with just a little work, it can be easily formatted into a CSS `rgba()` string in JavaScript.
/// This reference page doesn't carry information about the absolute color space that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB, DCI-P3, BT.2020, etc.).
/// By default, applications should assume the sRGB color space.
/// When color equality needs to be decided, implementations, unless documented otherwise, treat two colors as equal if all their red, green, blue, and alpha values each differ by at most 1e-5.
/// # Example (Java)
/// import com.google.type.Color; // ... public static java.awt.Color fromProto(Color protocolor) { float alpha = protocolor.hasAlpha() ? protocolor.getAlpha().getValue() : 1.0; return new java.awt.Color( protocolor.getRed(), protocolor.getGreen(), protocolor.getBlue(), alpha); } public static Color toProto(java.awt.Color color) { float red = (float) color.getRed(); float green = (float) color.getGreen(); float blue = (float) color.getBlue(); float denominator = 255.0; Color.Builder resultBuilder = Color .newBuilder() .setRed(red / denominator) .setGreen(green / denominator) .setBlue(blue / denominator); int alpha = color.getAlpha(); if (alpha != 255) { result.setAlpha( FloatValue .newBuilder() .setValue(((float) alpha) / denominator) .build()); } return resultBuilder.build(); } // ... Example (iOS / Obj-C): // ... static UIColor* fromProto(Color* protocolor) { float red = [protocolor red]; float green = [protocolor green]; float blue = [protocolor blue]; FloatValue* alpha_wrapper = [protocolor alpha]; float alpha = 1.0; if (alpha_wrapper != nil) { alpha = [alpha_wrapper value]; } return [UIColor colorWithRed:red green:green blue:blue alpha:alpha]; } static Color* toProto(UIColor* color) { CGFloat red, green, blue, alpha; if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) { return nil; } Color* result = [[Color alloc] init]; [result setRed:red]; [result setGreen:green]; [result setBlue:blue]; if (alpha \u003c= 0.9999) { [result setAlpha:floatWrapperWithValue(alpha)]; } [result autorelease]; return result; } // ... Example (JavaScript): // ... var protoToCssColor = function(rgb_color) { var redFrac = rgb_color.red || 0.0; var greenFrac = rgb_color.green || 0.0; var blueFrac = rgb_color.blue || 0.0; var red = Math.floor(redFrac * 255); var green = Math.floor(greenFrac * 255); var blue = Math.floor(blueFrac * 255); if (!('alpha' in rgb_color)) { return rgbToCssColor(red, green, blue); } var alphaFrac = rgb_color.alpha.value || 0.0; var rgbParams = [red, green, blue].join(','); return ['rgba(', rgbParams, ',', alphaFrac, ')'].join(''); }; var rgbToCssColor = function(red, green, blue) { var rgbNumber = new Number((red \u003c\u003c 16) | (green \u003c\u003c 8) | blue); var hexString = rgbNumber.toString(16); var missingZeros = 6 - hexString.length; var resultBuilder = ['#']; for (var i = 0; i \u003c missingZeros; i++) { resultBuilder.push('0'); } resultBuilder.push(hexString); return resultBuilder.join(''); }; // ...
#[derive(Serialize, Deserialize)]
pub struct Color {
    /// The fraction of this color that should be applied to the pixel.
    /// That is, the final pixel color is defined by the equation: `pixel color = alpha * (this color) + (1.0 - alpha) * (background color)`
    /// This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color.
    /// This uses a wrapper message rather than a simple float scalar so that it is possible to distinguish between a default value and the value being unset.
    /// If omitted, this color object is rendered as a solid color (as if the alpha value had been explicitly given a value of 1.0).
    alpha: f32,
    /// The amount of red in the color as a value in the interval [0, 1].
    red: f32,
    /// The amount of blue in the color as a value in the interval [0, 1].
    blue: f32,
    /// The amount of green in the color as a value in the interval [0, 1].
    green: f32,
}

/// The specifications of a slicer.
#[derive(Serialize, Deserialize)]
pub struct SlicerSpec {
    /// The background color of the slicer.
    #[serde(rename(serialize = "backgroundColor", deserialize = "backgroundColor"))]
    background_color: Color,
    /// The horizontal alignment of title in the slicer. If unspecified, defaults to `LEFT`
    horizontal_alignment: HorizontalAlignment,

    /// The column index in the data table on which the filter is applied to.
    #[serde(rename(serialize = "columnIndex", deserialize = "columnIndex"))]
    column_index: i32,

    /// The data range of the slicer.
    #[serde(rename(serialize = "dataRange", deserialize = "dataRange"))]
    data_range: GridRange,

    /// The background color of the slicer.
    /// If background_color is also set, this field takes precedence.
    #[serde(rename(serialize = "backgroundColorStyle", deserialize = "backgroundColorStyle"))]
    background_color_style: ColorStyle,

    /// The filtering criteria of the slicer.
    #[serde(rename(serialize = "filterCriteria", deserialize = "filterCriteria"))]
    filter_criteria: FilterCriteria,

    /// The title of the slicer.
    #[serde(rename(serialize = "title", deserialize = "title"))]
    title: String,

    /// True if the filter should apply to pivot tables. If not set, default to `True`.
    #[serde(rename(serialize = "applyToPivotTables", deserialize = "applyToPivotTables"))]
    apply_to_pivot_tables: bool,

    /// The text format of title in the slicer.
    /// The link field is not supported.
    #[serde(rename(serialize = "textFormat", deserialize = "textFormat"))]
    text_format: TextFormat,
}

/// A slicer in a sheet.
#[derive(Serialize, Deserialize)]
pub struct Slicer {
    /// The specification of the slicer.
    spec: SlicerSpec,

    /// The ID of the slicer.
    #[serde(rename(serialize = "slicerId", deserialize = "slicerId"))]
    slicer_id: i32,
    /// The position of the slicer.
    /// Note that slicer can be positioned only on existing sheet.
    /// Also, width and height of slicer can be automatically adjusted to keep it within permitted limits.
    position: EmbeddedObjectPosition,
}

/// Data about each cell in a row.
#[derive(Serialize, Deserialize)]
pub struct RowData {
    /// The values in the row, one per column.
    values: Vec<CellData>,
}

/// Properties about a dimension.
#[derive(Serialize, Deserialize)]
pub struct DimensionProperties {
    /// True if this dimension is being filtered. This field is read-only.
    #[serde(rename(serialize = "hiddenByFilter", deserialize = "hiddenByFilter"))]
    //#[readonly]
    hidden_by_filter: bool,

    /// Output only. If set, this is a column in a data source sheet.
    #[serde(rename(serialize = "dataSourceColumnReference", deserialize = "dataSourceColumnReference"))]
    //#[readonly]
    data_source_reference: DataSourceColumnReference,

    /// The developer metadata associated with a single row or column.
    #[serde(rename(serialize = "developerMetadata", deserialize = "developerMetadata"))]
    developer_metadata: Option<Vec<DeveloperMetadata>>,

    /// The height (if a row) or width (if a column) of the dimension in pixels.
    #[serde(rename(serialize = "pixelSize", deserialize = "pixelSize"))]
    pixel_size: i32,

    /// True if this dimension is explicitly hidden.
    #[serde(rename(serialize = "hiddenByUser", deserialize = "hiddenByUser"))]
    hidden_by_user: bool,
}

/// Data in the grid, as well as metadata about the dimensions.
#[derive(Serialize, Deserialize)]
pub struct GridData {
    /// Metadata about the requested rows in the grid, starting with the row in start_row.
    #[serde(rename(serialize = "rowMetadata", deserialize = "rowMetadata"))]
    row_metadata: Vec<DimensionProperties>,

    /// The first row this GridData refers to, zero-based.
    #[serde(rename(serialize = "startRow", deserialize = "startRow"))]
    start_row: i32,

    /// The first column this GridData refers to, zero-based.
    #[serde(rename(serialize = "startColumn", deserialize = "startColumn"))]
    start_column: i32,

    /// Metadata about the requested columns in the grid, starting with the column in start_column.
    #[serde(rename(serialize = "columnMetadata", deserialize = "columnMetadata"))]
    column_metadata: Vec<DimensionProperties>,

    /// The data in the grid, one entry per row, starting with the row in startRow.
    /// The values in RowData will correspond to columns starting at start_column.
    #[serde(rename(serialize = "rowData", deserialize = "rowData"))]
    row_data: Vec<RowData>,
}

/// A filter view.
#[derive(Serialize, Deserialize)]
pub struct FilterView {
    /// The named range this filter view is backed by, if any.
    /// When writing, only one of range or named_range_id may be set.
    #[serde(rename(serialize = "namedRangeId", deserialize = "namedRangeId"))]
    named_range_id: String,

    /// The range this filter view covers.
    /// When writing, only one of range or named_range_id may be set.
    range: GridRange,

    /// The name of the filter view.
    title: String,

    /// The filter criteria for showing/hiding values per column.
    /// Both criteria and filter_specs are populated in responses.
    /// If both fields are specified in an update request, this field takes precedence.
    #[serde(rename(serialize = "filterSpecs", deserialize = "filterSpecs"))]
    filter_specs: Vec<FilterSpec>,

    /// The sort order per column.
    /// Later specifications are used when values are equal in the earlier specifications.
    #[serde(rename(serialize = "sortSpecs", deserialize = "sortSpecs"))]
    sort_secs: Vec<SortSpec>,

    /// The ID of the filter view.
    #[serde(rename(serialize = "filterViewId", deserialize = "filterViewId"))]
    filter_view_id: i32,

    /// The criteria for showing/hiding values per column.
    /// The map's key is the column index, and the value is the criteria for that column.
    /// This field is deprecated in favor of filter_specs.
    criteria: FilterCriteria,
}

/// A sheet in a spreadsheet.
#[derive(Serialize, Deserialize)]
pub struct Sheet {
    /// The properties of the sheet.
    properties: SheetProperties,

    /// The protected ranges in this sheet.
    #[serde(rename(serialize = "protectedRanges", deserialize = "protectedRanges"))]
    protected_ranges: Option<Vec<ProtectedRange>>,

    /// The filter on this sheet, if any.
    #[serde(rename(serialize = "basicFilter", deserialize = "basicFilter"))]
    basic_filter: Option<BasicFilter>,

    /// All row groups on this sheet, ordered by increasing range start index, then by group depth.
    #[serde(rename(serialize = "rowGroups", deserialize = "rowGroups"))]
    row_groups: Option<Vec<DimensionGroup>>,

    /// The specifications of every chart on this sheet.
    charts: Vec<EmbeddedChart>,

    /// The developer metadata associated with a sheet.
    #[serde(rename(serialize = "developerMetadata", deserialize = "developerMetadata"))]
    developer_metadata: Option<Vec<DeveloperMetadata>>,

    /// The ranges that are merged together.
    merges: Option<Vec<GridRange>>,

    /// The conditional format rules in this sheet.
    #[serde(rename(serialize = "conditionalFormats", deserialize = "conditionalFormats"))]
    conditional_formats: Option<Vec<ConditionalFormatRule>>,

    /// All column groups on this sheet, ordered by increasing range start index, then by group depth.
    #[serde(rename(serialize = "columnGroups", deserialize = "columnGroups"))]
    column_groups: Option<Vec<DimensionGroup>>,

    /// The banded (alternating colors) ranges on this sheet.
    #[serde(rename(serialize = "bandedRanges", deserialize = "bandedRanges"))]
    banded_ranges: Option<Vec<BandedRange>>,

    /// The slicers on this sheet.
    slicers: Option<Vec<Slicer>>,

    /// Data in the grid, if this is a grid sheet. The number of GridData objects returned is dependent on the number of ranges requested on this sheet.
    /// For example, if this is representing `Sheet1`, and the spreadsheet was requested with ranges `Sheet1!A1:C10` and `Sheet1!D15:E20`, then the first GridData will have a startRow/startColumn of `0`, while the second one will have `startRow 14` (zero-based row 15), and `startColumn 3` (zero-based column D).
    /// For a DATA_SOURCE sheet, you can not request a specific range, the GridData contains all the values.
    data: Option<Vec<GridData>>,

    /// The filter views in this sheet.
    #[serde(rename(serialize = "filterViews", deserialize = "filterViews"))]
    filter_views: Option<Vec<FilterView>>,
}

/// An unique identifier that references a data source column.
#[derive(Serialize, Deserialize)]
pub struct DataSourceColumnReference {
    /// The display name of the column. It should be unique within a data source.
    name: String,
}

/// A column in a data source.
#[derive(Serialize, Deserialize)]
pub struct DataSourceColumn {
    /// The formula of the calculated column.
    formula: String,

    /// The column reference.
    reference: DataSourceColumnReference,
}

/// Specifies a BigQuery table definition.
/// Only [native tables](https://cloud.google.com/bigquery/docs/tables-intro) is allowed.
#[derive(Serialize, Deserialize)]
pub struct BigQueryTableSpec {
    /// The ID of a `BigQuery` project the table belongs to.
    /// If not specified, the project_id is assumed.
    #[serde(rename(serialize = "tableProjectId", deserialize = "tableProjectId"))]
    table_project_id: String,

    /// The `BigQuery` dataset id.
    #[serde(rename(serialize = "datasetId", deserialize = "datasetId"))]
    dataset_id: String,

    /// The `BigQuery` table id.
    #[serde(rename(serialize = "tableId", deserialize = "tableId"))]
    table_id: String,
}

/// The specification of a `BigQuery` data source that's connected to a sheet.
#[derive(Serialize, Deserialize)]
pub struct BigQueryDataSourceSpec {
    /// A `BigQueryQuerySpec`.
    #[serde(rename(serialize = "querySpec", deserialize = "querySpec"))]
    query_spec: BigQueryQuerySpec,

    /// The ID of a BigQuery enabled GCP project with a billing account attached.
    /// For any queries executed against the data source, the project is charged.
    #[serde(rename(serialize = "projectId", deserialize = "projectId"))]
    project_id: String,

    /// A `BigQueryTableSpec`.
    #[serde(rename(serialize = "tableSpec", deserialize = "tableSpec"))]
    table_spec: BigQueryTableSpec,
}

/// A parameter in a data source's query.
/// The parameter allows the user to pass in values from the spreadsheet into a query.
#[derive(Serialize, Deserialize)]
pub struct DataSourceParameter {
    /// Named parameter.
    /// Must be a legitimate identifier for the DataSource that supports it.
    /// For example, [BigQuery identifier](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#identifiers).
    name: String,
    /// ID of a NamedRange. Its size must be 1x1.
    #[serde(rename(serialize = "namedRangeId", deserialize = "namedRangeId"))]
    named_range_id: String,
    /// A range that contains the value of the parameter. Its size must be 1x1.
    range: GridRange,
}

/// This specifies the details of the data source.
/// For example, for BigQuery, this specifies information about the BigQuery source.
#[derive(Serialize, Deserialize)]
pub struct DataSourceSpec {
    /// A `BigQueryDataSourceSpec`.
    big_query: BigQueryDataSourceSpec,
    /// The parameters of the data source, used when querying the data source.
    params: Vec<DataSourceParameter>,
}

/// Information about an external data source in the spreadsheet.
#[derive(Serialize, Deserialize)]
pub struct DataSource {
    /// All calculated columns in the data source.
    #[serde(rename(serialize = "calculatedColumns", deserialize = "calculatedColumns"))]
    calculated_columns: Vec<DataSourceColumn>,

    /// The ID of the Sheet connected with the data source.
    /// The field cannot be changed once set.
    /// When creating a data source, an associated DATA_SOURCE sheet is also created, if the field is not specified, the ID of the created sheet will be randomly generated.
    #[serde(rename(serialize = "sheetId", deserialize = "sheetId"))]
    sheet_id: i32,

    /// The DataSourceSpec for the data source connected with this spreadsheet.
    spec: DataSourceSpec,
    /// The spreadsheet-scoped unique ID that identifies the data source.
    /// Example: 1080547365.
    #[serde(rename(serialize = "dataSourceId", deserialize = "dataSourceId"))]
    data_source_id: String,
}

/// A range along a single dimension on a sheet.
/// All indexes are zero-based.
/// Indexes are half open: the start index is inclusive and the end index is exclusive.
/// Missing indexes indicate the range is unbounded on that side.
#[derive(Serialize, Deserialize)]
pub struct DimensionRange {
    /// The end (exclusive) of the span, or not set if unbounded.
    #[serde(rename(serialize = "endIndex", deserialize = "endIndex"))]
    end_index: i32,
    /// The sheet this span is on.
    #[serde(rename(serialize = "sheetId", deserialize = "sheetId"))]
    sheet_id: i32,
    /// The start (inclusive) of the span, or not set if unbounded.
    #[serde(rename(serialize = "startIndex", deserialize = "startIndex"))]
    start_index: i32,
}

/// A location where metadata may be associated in a spreadsheet.
#[derive(Serialize, Deserialize)]
pub struct DeveloperMetadataLocation {
    /// Represents the row or column when metadata is associated with a dimension.
    /// The specified DimensionRange must represent a single row or column;
    /// it cannot be unbounded or span multiple rows or columns.
    #[serde(rename(serialize = "dimensionRange", deserialize = "dimensionRange"))]
    dimension_range: DimensionRange,

    /// True when metadata is associated with an entire spreadsheet.
    spreadsheet: bool,

    /// The type of location this object represents. This field is read-only.
    #[serde(rename(serialize = "locationType", deserialize = "locationType"))]
    location_type: LocationType, //#[readonly]

    /// The ID of the sheet when metadata is associated with an entire sheet.
    #[serde(rename(serialize = "sheetId", deserialize = "sheetId"))]
    sheet_id: i32,
}

/// Developer metadata associated with a location or object in a spreadsheet.
/// Developer metadata may be used to associate arbitrary data with various parts of a spreadsheet and will remain associated at those locations as they move around and the spreadsheet is edited. For example, if developer metadata is associated with row 5 and another row is then subsequently inserted above row 5, that original metadata will still be associated with the row it was first associated with (what is now row 6). If the associated object is deleted its metadata is deleted too.
#[derive(Serialize, Deserialize)]
pub struct DeveloperMetadata {
    /// The metadata key.
    /// There may be multiple metadata in a spreadsheet with the same key.
    /// Developer metadata must always have a key specified.
    #[serde(rename(serialize = "metadataKey", deserialize = "metadataKey"))]
    metadata_key: String,
    /// The spreadsheet-scoped unique ID that identifies the metadata.
    /// IDs may be specified when metadata is created, otherwise one will be randomly generated and assigned.
    /// Must be positive.
    #[serde(rename(serialize = "metadataId", deserialize = "metadataId"))]
    metadata_id: i32,
    /// The location where the metadata is associated.
    location: DeveloperMetadataLocation,
    /// Data associated with the metadata's key.
    #[serde(rename(serialize = "metadataValue", deserialize = "metadataValue"))]
    metadata_value: String,
    /// The metadata visibility.
    /// Developer metadata must always have a visibility specified.
    visibility: Visibility,
}

/// A schedule for data to refresh every day in a given time interval.
#[derive(Serialize, Deserialize)]
pub struct DataSourceRefreshDailySchedule {
    #[serde(rename(serialize = "startTime", deserialize = "startTime"))]
    /// The start time of a time interval in which a data source refresh is scheduled.
    /// Only `hours` part is used. The time interval size defaults to that in the Sheets editor.
    start_time: TimeOfDay,
}

/// Represents a time of day.
/// The date and time zone are either not significant or are specified elsewhere.
/// An API may choose to allow leap seconds.
/// Related types are google.type.Date and `google.protobuf.Timestamp`.
#[derive(Serialize, Deserialize)]
pub struct TimeOfDay {
    /// Minutes of hour of day.
    /// Must be from 0 to 59.
    minutes: i32,
    /// Hours of day in 24 hour format.
    /// Should be from 0 to 23.
    /// An API may choose to allow the value \"24:00:00\" for scenarios like business closing time.
    hours: i32,
    /// Seconds of minutes of the time.
    /// Must normally be from 0 to 59.
    /// An API may allow the value 60 if it allows leap-seconds.
    seconds: i32,
    /// Fractions of seconds in nanoseconds.
    /// Must be from 0 to 999,999,999.
    nanos: i32,
}

/// A monthly schedule for data to refresh on specific days in the month in a given time interval.
#[derive(Serialize, Deserialize)]
pub struct DataSourceRefreshMonthlySchedule {
    #[serde(rename(serialize = "daysOfMonth", deserialize = "daysOfMonth"))]
    /// Days of the month to refresh.
    /// Only 1-28 are supported, mapping to the 1st to the 28th day.
    /// At least one day must be specified.
    days_of_month: Vec<i32>,

    #[serde(rename(serialize = "startTime", deserialize = "startTime"))]
    /// The start time of a time interval in which a data source refresh is scheduled.
    /// Only `hours` part is used.
    /// The time interval size defaults to that in the Sheets editor.
    start_time: TimeOfDay,
}

/// A weekly schedule for data to refresh on specific days in a given time interval.
#[derive(Serialize, Deserialize)]
pub struct DataSourceRefreshWeeklySchedule {
    /// The start time of a time interval in which a data source refresh is scheduled.
    /// Only `hours` part is used.
    /// The time interval size defaults to that in the Sheets editor.
    #[serde(rename(serialize = "startTime", deserialize = "startTime"))]
    start_time: TimeOfDay,
    /// Days of the week to refresh.
    /// At least one day must be specified.
    #[serde(rename(serialize = "dailySchedule", deserialize = "dailySchedule"))]
    days_of_week: Vec<DayOfWeek>,
}

/// Represents a time interval, encoded as a Timestamp start (inclusive) and a Timestamp end (exclusive).
/// The start must be less than or equal to the end.
/// When the start equals the end, the interval is empty (matches no time).
/// When both start and end are unspecified, the interval matches any time.
#[derive(Serialize, Deserialize)]
pub struct Interval {
    #[serde(rename(serialize = "startTime", deserialize = "startTime"))]
    /// Optional. Inclusive start of the interval.
    /// If specified, a Timestamp matching this interval will have to be the same or after the start.
    start_time: Option<String>,//google-datetime

    #[serde(rename(serialize = "endTime", deserialize = "endTime"))]
    /// Exclusive end of the interval.
    /// If specified, a Timestamp matching this interval will have to be before the end.
    end_time: Option<String>,//google-datetime
}

/// Schedule for refreshing the data source.
/// Data sources in the spreadsheet are refreshed within a time interval.
/// You can specify the start time by clicking the Scheduled Refresh button in the Sheets editor, but the interval is fixed at 4 hours.
/// For example, if you specify a start time of 8am , the refresh will take place between 8am and 12pm every day.
#[derive(Serialize, Deserialize)]
pub struct DataSourceRefreshSchedule {
    /// True if the refresh schedule is enabled, or false otherwise.
    enabled: bool,

    #[serde(rename(serialize = "dailySchedule", deserialize = "dailySchedule"))]
    /// Daily refresh schedule.
    daily_schedule: DataSourceRefreshDailySchedule,

    #[serde(rename(serialize = "monthlySchedule", deserialize = "monthlySchedule"))]
    /// Monthly refresh schedule.
    monthly_schedule: DataSourceRefreshMonthlySchedule,

    #[serde(rename(serialize = "weeklySchedule", deserialize = "weeklySchedule"))]
    //#[readonly]
    /// Output only. The time interval of the next run.
    next_run: Interval,

    #[serde(rename(serialize = "weeklySchedule", deserialize = "weeklySchedule"))]
    /// Weekly refresh schedule.
    weekly_schedule: DataSourceRefreshWeeklySchedule,

    #[serde(rename(serialize = "refreshScope", deserialize = "refreshScope"))]
    /// The scope of the refresh. Must be ALL_DATA_SOURCES.
    refresh_scope: RefreshScope,
}

/// A named range.
#[derive(Serialize, Deserialize)]
pub struct NamedRange {
    #[serde(rename(serialize = "namedRangeId", deserialize = "namedRangeId"))]
    /// The ID of the named range.
    named_range_id: String,

    #[serde(rename(serialize = "name", deserialize = "name"))]
    /// The name of the named range.
    name: String,

    #[serde(rename(serialize = "range", deserialize = "range"))]
    /// The range this represents.
    range: GridRange,
}

/// A range on a sheet. All indexes are zero-based.
/// Indexes are half open, i.e. the start index is inclusive and the end index is exclusive -- [start_index, end_index).
/// Missing indexes indicate the range is unbounded on that side.
/// For example, if `\"Sheet1\"` is sheet ID 0, then: `Sheet1!A1:A1 == sheet_id: 0, start_row_index: 0, end_row_index: 1, start_column_index: 0, end_column_index: 1` `Sheet1!A3:B4 == sheet_id: 0, start_row_index: 2, end_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1!A:B == sheet_id: 0, start_column_index: 0, end_column_index: 2` `Sheet1!A5:B == sheet_id: 0, start_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1 == sheet_id:0`
/// The start index must always be less than or equal to the end index.
/// If the start index equals the end index, then the range is empty.
/// Empty ranges are typically not meaningful and are usually rendered in the UI as `#REF!`.
#[derive(Serialize, Deserialize)]
pub struct GridRange {
    #[serde(rename(serialize = "endColumnIndex", deserialize = "endColumnIndex"))]
    /// The end column (exclusive) of the range, or not set if unbounded.
    end_column_index: i32,
    #[serde(rename(serialize = "endRowIndex", deserialize = "endRowIndex"))]
    /// The end row (exclusive) of the range, or not set if unbounded.
    end_row_index: i32,
    #[serde(rename(serialize = "sheetId", deserialize = "sheetId"))]
    /// The sheet this range is on.
    sheet_id: i32,
    #[serde(rename(serialize = "startRowIndex", deserialize = "startRowIndex"))]
    /// The start row (inclusive) of the range, or not set if unbounded.
    start_row_index: i32,
    #[serde(rename(serialize = "startColumnIndex", deserialize = "startColumnIndex"))]
    /// The start column (inclusive) of the range, or not set if unbounded.
    start_column_index: i32,
}
