pub mod tool;
pub mod models;
pub mod enums;

use serde_json::json;
use std::fmt::{Display, Formatter};
use super::sheet::models::*;

pub struct SheetApi {}

impl SheetApi {
    pub fn new() -> Self {
        Self {}
    }

    // pub fn create() -> Spreadsheet {
    //     Spreadsheet {}
    // }
}

//<editor-fold desc="Tests">

#[test]
fn test_spreadsheet_decode() {
    let json_value = json!({
  "spreadsheetId": "1Zzq6vF93SJsd1iNnLdxtA2Yg3SP115Mtli4UYGeJD4A",
  "properties": {
    "title": "Новая таблица",
    "locale": "ru_RU",
    "autoRecalc": "ON_CHANGE",
    "timeZone": "Europe/Paris",
    "defaultFormat": {
      "backgroundColor": {
        "red": 1,
        "green": 1,
        "blue": 1
      },
      "padding": {
        "top": 2,
        "right": 3,
        "bottom": 2,
        "left": 3
      },
      "verticalAlignment": "BOTTOM",
      "wrapStrategy": "OVERFLOW_CELL",
      "textFormat": {
        "foregroundColor": {},
        "fontFamily": "arial,sans,sans-serif",
        "fontSize": 10,
        "bold": false,
        "italic": false,
        "strikethrough": false,
        "underline": false,
        "foregroundColorStyle": {
          "rgbColor": {}
        }
      },
      "backgroundColorStyle": {
        "rgbColor": {
          "red": 1,
          "green": 1,
          "blue": 1
        }
      }
    },
    "spreadsheetTheme": {
      "primaryFontFamily": "Arial",
      "themeColors": [
        {
          "colorType": "ACCENT3",
          "color": {
            "rgbColor": {
              "red": 0.9843137,
              "green": 0.7372549,
              "blue": 0.015686275
            }
          }
        },
        {
          "colorType": "ACCENT1",
          "color": {
            "rgbColor": {
              "red": 0.25882354,
              "green": 0.52156866,
              "blue": 0.95686275
            }
          }
        },
        {
          "colorType": "BACKGROUND",
          "color": {
            "rgbColor": {
              "red": 1,
              "green": 1,
              "blue": 1
            }
          }
        },
        {
          "colorType": "ACCENT4",
          "color": {
            "rgbColor": {
              "red": 0.20392157,
              "green": 0.65882355,
              "blue": 0.3254902
            }
          }
        },
        {
          "colorType": "TEXT",
          "color": {
            "rgbColor": {}
          }
        },
        {
          "colorType": "LINK",
          "color": {
            "rgbColor": {
              "red": 0.06666667,
              "green": 0.33333334,
              "blue": 0.8
            }
          }
        },
        {
          "colorType": "ACCENT6",
          "color": {
            "rgbColor": {
              "red": 0.27450982,
              "green": 0.7411765,
              "blue": 0.7764706
            }
          }
        },
        {
          "colorType": "ACCENT5",
          "color": {
            "rgbColor": {
              "red": 1,
              "green": 0.42745098,
              "blue": 0.003921569
            }
          }
        },
        {
          "colorType": "ACCENT2",
          "color": {
            "rgbColor": {
              "red": 0.91764706,
              "green": 0.2627451,
              "blue": 0.20784314
            }
          }
        }
      ]
    }
  },
  "sheets": [
    {
      "properties": {
        "sheetId": 0,
        "title": "Лист1",
        "index": 0,
        "sheetType": "GRID",
        "gridProperties": {
          "rowCount": 1000,
          "columnCount": 27
        }
      },
      "charts": [
        {
          "chartId": 1611000291,
          "spec": {
            "pieChart": {
              "domain": {
                "sourceRange": {
                  "sources": [
                    {
                      "startRowIndex": 0,
                      "endRowIndex": 104,
                      "startColumnIndex": 1,
                      "endColumnIndex": 2
                    }
                  ]
                }
              },
              "series": {
                "sourceRange": {
                  "sources": [
                    {
                      "startRowIndex": 0,
                      "endRowIndex": 104,
                      "startColumnIndex": 3,
                      "endColumnIndex": 4
                    }
                  ]
                }
              },
              "pieHole": 0.5
            },
            "hiddenDimensionStrategy": "SKIP_HIDDEN_ROWS_AND_COLUMNS",
            "fontName": "Roboto"
          },
          "position": {
            "overlayPosition": {
              "anchorCell": {
                "rowIndex": 1,
                "columnIndex": 5
              },
              "offsetXPixels": 12,
              "offsetYPixels": 5,
              "widthPixels": 971,
              "heightPixels": 601
            }
          }
        },
        {
          "chartId": 1078085974,
          "spec": {
            "pieChart": {
              "domain": {
                "sourceRange": {
                  "sources": [
                    {
                      "startRowIndex": 0,
                      "endRowIndex": 104,
                      "startColumnIndex": 2,
                      "endColumnIndex": 3
                    }
                  ]
                }
              },
              "series": {
                "sourceRange": {
                  "sources": [
                    {
                      "startRowIndex": 0,
                      "endRowIndex": 104,
                      "startColumnIndex": 3,
                      "endColumnIndex": 4
                    }
                  ]
                }
              },
              "pieHole": 0.5
            },
            "hiddenDimensionStrategy": "SKIP_HIDDEN_ROWS_AND_COLUMNS",
            "fontName": "Roboto"
          },
          "position": {
            "overlayPosition": {
              "anchorCell": {
                "rowIndex": 32,
                "columnIndex": 5
              },
              "offsetXPixels": 12,
              "offsetYPixels": 1,
              "widthPixels": 600,
              "heightPixels": 371
            }
          }
        }
      ]
    }
  ],
  "spreadsheetUrl": "https://docs.google.com/spreadsheets/d/1Zzq6vF93SJsd1iNnLdxtA2Yg3SP115Mtli4UYGeJD4A/edit"
}
  );

    let spreadsheet: Spreadsheet = serde_json::from_value(json_value).unwrap();

    assert_eq!(spreadsheet.spreadsheet_id(), "1Zzq6vF93SJsd1iNnLdxtA2Yg3SP115Mtli4UYGeJD4A");
    assert_eq!(spreadsheet.spreadsheet_url(), "https://docs.google.com/spreadsheets/d/1Zzq6vF93SJsd1iNnLdxtA2Yg3SP115Mtli4UYGeJD4A/edit");
    assert_eq!(spreadsheet.sheets.len(), 1);
}
//</editor-fold">
