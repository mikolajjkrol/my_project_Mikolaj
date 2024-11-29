use std::cell::RefCell;

thread_local! {
    static CZAT:  RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn save_msg(msg: String) {
        CZAT.with(|msgs| msgs.borrow_mut().push(msg));
}

#[ic_cdk::query]
fn get_chat() -> Vec<String> {
        CZAT.with(|msgs| msgs.borrow().clone())
}