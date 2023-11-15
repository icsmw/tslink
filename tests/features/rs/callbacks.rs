use tslink::tslink;

#[tslink]
fn callback_generic_a<F: Fn(i32, i32, bool)>(callback: F) {
    callback(666, 666, true);
}

#[tslink]
fn callback_generic_b<F: Fn(i32, i32, bool) -> i32>(callback: F) {
    callback(666, 666, true);
}

#[tslink]
fn callback_generic_c<F: Fn(i32, i32, bool) -> i32 + Send + 'static>(callback: F) {
    callback(666, 666, true);
}

#[tslink]
fn callback_generic_d<F>(callback: F)
where
    F: Fn(i32, i32, bool),
{
    callback(666, 666, true);
}

#[tslink]
fn callback_generic_e<F>(callback: F)
where
    F: Fn(i32, i32, bool) -> String,
{
    callback(666, 666, true);
}

#[tslink]
fn callback_generic_f<F>(callback: F)
where
    F: Fn(i32, i32, bool) -> String + Send + 'static,
{
    callback(666, 666, true);
}

struct CallbacksStruct {}

#[tslink(class)]
impl CallbacksStruct {
    #[tslink]
    fn callback_generic_a_mut<F: Fn(i32, i32, bool)>(&mut self, callback: F) {
        callback(666, 666, true);
    }

    #[tslink]
    fn callback_generic_b_mut<F: Fn(i32, i32, bool) -> i32>(&mut self, callback: F) {
        callback(666, 666, true);
    }

    #[tslink]
    fn callback_generic_c_mut<F: Fn(i32, i32, bool) -> i32 + Send + 'static>(
        &mut self,
        callback: F,
    ) {
        callback(666, 666, true);
    }

    #[tslink]
    fn callback_generic_d_mut<F>(&mut self, callback: F)
    where
        F: Fn(i32, i32, bool),
    {
        callback(666, 666, true);
    }

    #[tslink]
    fn callback_generic_e_mut<F>(&mut self, callback: F)
    where
        F: Fn(i32, i32, bool) -> String,
    {
        callback(666, 666, true);
    }

    #[tslink]
    fn callback_generic_f_mut<F>(&mut self, callback: F)
    where
        F: Fn(i32, i32, bool) -> String + Send + 'static,
    {
        callback(666, 666, true);
    }

    #[tslink]
    fn callback_generic_a<F: Fn(i32, i32, bool)>(&self, callback: F) {
        callback(666, 666, true);
    }

    #[tslink]
    fn callback_generic_b<F: Fn(i32, i32, bool) -> i32>(&self, callback: F) {
        callback(666, 666, true);
    }

    #[tslink]
    fn callback_generic_c<F: Fn(i32, i32, bool) -> i32 + Send + 'static>(&self, callback: F) {
        callback(666, 666, true);
    }

    #[tslink]
    fn callback_generic_d<F>(&self, callback: F)
    where
        F: Fn(i32, i32, bool),
    {
        callback(666, 666, true);
    }

    #[tslink]
    fn callback_generic_e<F>(&self, callback: F)
    where
        F: Fn(i32, i32, bool) -> String,
    {
        callback(666, 666, true);
    }

    #[tslink]
    fn callback_generic_f<F>(&self, callback: F)
    where
        F: Fn(i32, i32, bool) -> String + Send + 'static,
    {
        callback(666, 666, true);
    }
}
