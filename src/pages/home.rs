use gloo_worker::Spawnable;
use leptos::html::*;
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use web_sys::SubmitEvent;

use crate::{GetFromDictionaryWorker, HelperStruct};

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx, <HomeContent/> }
}

#[component]
pub fn HomeContent(cx: Scope) -> impl IntoView {
    let (anagram_letters, set_anagram_letters) = create_signal(cx, "".to_string());
    let (magic_letter, set_magic_letter) = create_signal::<char>(cx, 'A');
    let (letter_count, set_letter_count) = create_signal(cx, 4usize);

    let (found_word_amount, set_found_word_amount) = create_signal(cx, 0usize);

    let (anagram_output, set_anagram_output) = create_signal::<Vec<String>>(cx, vec![]);

    let anagram_input_element: NodeRef<Input> = create_node_ref(cx);
    let letter_input_element: NodeRef<Input> = create_node_ref(cx);
    
    
    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();

        let value = anagram_input_element().unwrap().value();

        set_anagram_letters.set(value);
        
        let mut bridge = GetFromDictionaryWorker::spawner().spawn("/worker.js");
        
        spawn_local(async move {
            let mut output_words = bridge.run(
                HelperStruct {
                    reference: anagram_letters.get_untracked().to_string(),
                    magic_letter: magic_letter.get_untracked(),
                    letter_count: letter_count.get_untracked(),
                }
            ).await;
            
            output_words.sort();
            
            set_found_word_amount(output_words.len());
            set_anagram_output.set(output_words);
        });
    };

    view! { cx,
        <div class="flex-1 flex items-start justify-center sm:p-8 py-8 px-3 fade-in-transition">
            <div class="flex flex-col items-center justify-center shadow-md bg-gray-100 w-full sm:p-5 py-5 px-2">
                <div class="flex flex-col">
                    <form class="flex flex-col" on:submit=on_submit>
                        <label class="label" for="">
                            "Minimum Letter Count"
                        </label>
                        <input
                            class="input mb-4"
                            type="number"
                            value=letter_count
                            node_ref=letter_input_element
                            on:input=move |_| {
                                let letter_input = letter_input_element().unwrap().value();
                                set_letter_count(letter_input.parse().unwrap_or(4));
                            }
                        />
                        <label class="label" for="">
                            "Magic Letter"
                        </label>
                        <input
                            class="input mb-4"
                            type="text"
                            value=magic_letter
                            on:input=move |e| {
                                let magic_letter_input = e
                                    .target()
                                    .unwrap()
                                    .unchecked_into::<HtmlInputElement>()
                                    .value();
                                set_magic_letter(magic_letter_input.parse().unwrap_or('A'));
                            }
                        />
                        <label class="label" for="">
                            "Letters"
                        </label>
                        <input
                            class="input mb-4"
                            type="text"
                            value=anagram_letters
                            node_ref=anagram_input_element
                        />
                        <input
                            class="btn bg-purple-300 text-white hover:text-green-600 active:text-green-400 active:bg-gray-200 mb-4"
                            type="submit"
                            value="SUBMIT"
                        />
                    </form>
                    <h2 class="text-xl text-orange-400">
                        "Found Word Count: "
                        <span class="text-orange-400 font-bold">{found_word_amount}</span>
                    </h2>
                    <ul class="mt-4">
                        {move || {
                            anagram_output()
                                .into_iter()
                                .map(|value| {
                                    view! { cx,
                                            <li class="flex items-center">
                                                <div class="flex-1">
                                                    <input class="checkbox checkbox-success" type="checkbox"/>
                                                </div>
                                                {value}
                                            </li>
                                    }
                                })
                                .collect_view(cx)
                        }}
                    </ul>
                </div>
            </div>
        </div>
    }
}