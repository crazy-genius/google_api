use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

//region Dimension
/// The metadata visibility.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dimension {
    DimensionUnspecified,
    Rows,
    Columns,
}

impl Into<&str> for Dimension {
    fn into(self) -> &'static str {
        match self {
            Self::Rows => "ROWS",
            Self::Columns => "COLUMNS",
            Self::DimensionUnspecified => "DIMENSION_UNSPECIFIED",
        }
    }
}

impl From<&str> for Dimension {
    fn from(text: &str) -> Self {
        match text {
            "ROWS" => Self::Rows,
            "COLUMNS" => Self::Columns,
            _ => Self::DimensionUnspecified,
        }
    }
}

impl Display for Dimension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rows => write!(f, "Operates on the rows of a sheet."),
            Self::Columns => write!(f, "Operates on the columns of a sheet."),
            _ => write!(f, "The default value, do not use."),
        }
    }
}
//endregion

//region Visibility
/// The metadata visibility.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Visibility {
    DeveloperMetadataVisibilityUnspecified,
    Document,
    Project,
}

impl Display for Visibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Document => write!(f, "DOCUMENT"),
            Self::Project => write!(f, "PROJECT"),
            _ => write!(f, "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED"),
        }
    }
}

impl Into<&str> for Visibility {
    fn into(self) -> &'static str {
        match self {
            Self::Document => "DOCUMENT",
            Self::Project => "PROJECT",
            _ => "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED",
        }
    }
}

impl From<&str> for Visibility {
    fn from(text: &str) -> Self {
        match text {
            "DOCUMENT" => Self::Document,
            "PROJECT" => Self::Project,
            _ => Self::DeveloperMetadataVisibilityUnspecified,
        }
    }
}
//endregion

//region LocationType
/// The type of location
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum LocationType {
    Row,
    Column,
    Sheet,
    SpreadSheet,
    DeveloperMetadataLocationTypeUnspecified,
}

impl Into<&str> for LocationType {
    fn into(self) -> &'static str {
        match self {
            Self::Row => "ROW",
            Self::Column => "COLUMN",
            Self::Sheet => "SHEET",
            Self::SpreadSheet => "SPREADSHEET",
            Self::DeveloperMetadataLocationTypeUnspecified => "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED",
        }
    }
}

impl From<&str> for LocationType {
    fn from(text: &str) -> Self {
        match text {
            "ROW" => Self::Row,
            "COLUMN" => Self::Column,
            "SHEET" => Self::Sheet,
            "SPREADSHEET" => Self::SpreadSheet,
            _ => Self::DeveloperMetadataLocationTypeUnspecified,
        }
    }
}

impl Display for LocationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Row => write!(f, "Developer metadata associated on an entire row dimension."),
            Self::Column => write!(f, "Developer metadata associated on an entire column dimension."),
            Self::Sheet => write!(f, "Developer metadata associated on an entire sheet."),
            Self::SpreadSheet => write!(f, "Developer metadata associated on the entire spreadsheet."),
            _ => write!(f, "Default value."),
        }
    }
}
//endregion

//region RefreshScope
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RefreshScope {
    /// Default value, do not use.
    DataSourceRefreshScopeUnspecified,
    /// Refreshes all data sources and their associated data source objects in the spreadsheet.
    AllDataSources,
}

impl Display for RefreshScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DataSourceRefreshScopeUnspecified => write!(f, "DATA_SOURCE_REFRESH_SCOPE_UNSPECIFIED"),
            Self::AllDataSources => write!(f, "ALL_DATA_SOURCES"),
        }
    }
}

impl From<&str> for RefreshScope {
    fn from(text: &str) -> Self {
        match text {
            "ALL_DATA_SOURCES" => RefreshScope::AllDataSources,
            _ => RefreshScope::DataSourceRefreshScopeUnspecified,
        }
    }
}
//endregion

//region DayOfWeek
/// Days of the week to refresh.
/// At least one day must be specified.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DayOfWeek {
    DayOfWeekUnspecified,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl From<&str> for DayOfWeek {
    fn from(text: &str) -> Self {
        match text {
            "Monday" => Self::Monday,
            "Tuesday" => Self::Thursday,
            "Wednesday" => Self::Wednesday,
            "Thursday" => Self::Thursday,
            "Friday" => Self::Friday,
            "Saturday" => Self::Saturday,
            "Sunday" => Self::Sunday,
            _ => Self::DayOfWeekUnspecified,
        }
    }
}

impl Display for DayOfWeek {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Monday => write!(f, "Monday"),
            Self::Tuesday => write!(f, "Tuesday"),
            Self::Wednesday => write!(f, "Wednesday"),
            Self::Thursday => write!(f, "Thursday"),
            Self::Friday => write!(f, "Friday"),
            Self::Saturday => write!(f, "Saturday"),
            Self::Sunday => write!(f, "Sunday"),
            _ => write!(f, "The day of the week is unspecified."),
        }
    }
}
//endregion

//region HorizontalAlignment
/// The horizontal alignment of title in the slicer.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HorizontalAlignment {
    HorizontalAlignUnspecified,
    Left,
    Center,
    Right,
}

impl Into<&str> for HorizontalAlignment {
    fn into(self) -> &'static str {
        match self {
            Self::Left => "LEFT",
            Self::Center => "CENTER",
            Self::Right => "RIGHT",
            Self::HorizontalAlignUnspecified => "HORIZONTAL_ALIGN_UNSPECIFIED",
        }
    }
}

impl Display for HorizontalAlignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Right => write!(f, "The text is explicitly aligned to the right of the cell."),
            Self::Center => write!(f, "The text is explicitly aligned to the center of the cell."),
            Self::Left => write!(f, "The text is explicitly aligned to the left of the cell."),
            Self::HorizontalAlignUnspecified => write!(f, "The horizontal alignment is not specified. Do not use this."),
        }
    }
}
//endregion

//region ThemeColor
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThemeColor {
    ThemColorTypeUnspecified,
    Text,
    Background,
    ACCENT1,
    ACCENT2,
    ACCENT3,
    ACCENT4,
    ACCENT5,
    ACCENT6,
    Link,
}

impl Into<&str> for ThemeColor {

    fn into(self) -> &'static str {
        match self {
            Self::ThemColorTypeUnspecified => "THEME_COLOR_TYPE_UNSPECIFIED",
            Self::Text => "TEXT",
            Self::Background => "BACKGROUND",
            Self::ACCENT1 => "ACCENT1",
            Self::ACCENT2 => "ACCENT2",
            Self::ACCENT3 => "ACCENT3",
            Self::ACCENT4 => "ACCENT4",
            Self::ACCENT5 => "ACCENT5",
            Self::ACCENT6 => "ACCENT6",
            Self::Link => "LINK",
        }
    }
}

impl Display for ThemeColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ThemColorTypeUnspecified => write!(f, "Unspecified theme color"),
            Self::Text => write!(f, "Represents the primary text color"),
            Self::Background => write!(f, "Represents the primary background color"),
            Self::ACCENT1 => write!(f, "Represents the first accent color"),
            Self::ACCENT2 => write!(f, "Represents the second accent color"),
            Self::ACCENT3 => write!(f, "Represents the third accent color"),
            Self::ACCENT4 => write!(f, "Represents the fourth accent color"),
            Self::ACCENT5 => write!(f, "Represents the fifth accent color"),
            Self::ACCENT6 => write!(f, "Represents the sixth accent color"),
            Self::Link => write!(f, "Represents the color to use for hyperlinks"),
        }
    }
}
//endregion