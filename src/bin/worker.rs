use gloo_worker::Registrable;
use anagram_solver::GetFromDictionaryWorker;

fn main() {
    console_error_panic_hook::set_once();
    GetFromDictionaryWorker::registrar().register();
}