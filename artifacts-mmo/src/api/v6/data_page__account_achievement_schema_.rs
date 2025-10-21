#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DataPage_AccountAchievementSchema_ {
    /// Data
    data: Vec<super::account_achievement_schema::AccountAchievementSchema>,
    /// Page
    page: i32,
    /// Pages
    pages: i32,
    /// Size
    size: i32,
    /// Total
    total: i32,
}
