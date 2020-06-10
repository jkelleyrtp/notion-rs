use {super::notionclient, std::collections::HashMap};

enum CollectionView {
    Table,
    Calendar,
    List,
    Board,
}

enum ErrorType {
    is_private,
    invalid_id,
    none,
}

pub struct Collection {
    blocks: HashMap<String, String>,
    view_type: CollectionView,
    error_type: ErrorType,
}
