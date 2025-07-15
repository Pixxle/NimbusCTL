use crate::config::user_config::UserConfig;

pub fn get_default_config() -> UserConfig {
    UserConfig::default()
}

pub fn get_default_keybindings() -> Vec<(&'static str, &'static str)> {
    vec![
        ("q", "Quit"),
        ("?", "Help"),
        ("Ctrl+p", "Quick navigation"),
        ("h", "Home/Dashboard"),
        ("Ctrl+r", "Region selector"),
        ("F2", "Settings"),
        ("Tab", "Navigate widgets"),
        ("Enter", "Select"),
        ("Esc", "Back"),
        ("↑↓", "Navigate"),
        ("←→", "Navigate"),
        ("1-9", "Quick actions"),
        ("Space", "Toggle favorite"),
        ("/", "Search"),
        ("c", "Create"),
        ("e", "Edit"),
        ("d", "Delete"),
        ("r", "Refresh"),
    ]
}

pub fn get_default_dashboard_widgets() -> Vec<&'static str> {
    vec![
        "favorites",
        "recent",
        "quick_actions",
        "region_overview",
        "service_status",
    ]
}

pub fn get_default_quick_actions() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Launch EC2 Instance", "ec2_launch"),
        ("Create S3 Bucket", "s3_create"),
        ("Create RDS Database", "rds_create"),
        ("Create IAM User", "iam_create"),
        ("Create Secret", "secrets_create"),
    ]
}
