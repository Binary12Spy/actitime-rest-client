use crate::payload::BatchRequestItem;

pub enum PostBatchRequestsIncludeResponseBody {
    Always,
    Never,
    Get,
}

// pub fn post_batch_requests(
//     requests: Vec<Box<dyn BatchRequestItem>>,
// ) -> Route<(), Vec<BatchResultItem>> {
//     let url = format!("/leaveTypes/{}", id);
//     Route::new(Method::GET, &url)
// }
