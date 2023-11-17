use leptos::{component, view, IntoAttribute, IntoView, Scope};
use serde_derive::{Deserialize, Serialize};

#[component]
pub fn ChatSideBar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col py-8 pl-6 pr-2 w-64 bg-white flex-shrink-0">
            <div class="flex flex-row items-center justify-center h-12 w-full">
                <div class="flex items-center justify-center rounded-2xl text-indigo-700 bg-indigo-100 h-10 w-10">
                    <svg
                        class="w-6 h-6"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z"
                        ></path>
                    </svg>
                </div>
                <div class="ml-2 font-bold text-xl">"D&D AI Assistant"</div>
            </div>
            <div class="flex flex-col mt-8">
                <div class="flex flex-row items-center justify-between text-xs">
                    <span class="font-bold">Game Sessions</span>
                    <span class="flex items-center justify-center bg-gray-300 h-4 w-4 rounded-full">
                        2
                    </span>
                </div>
                <div class="flex flex-col space-y-1 mt-4 -mx-2 h-48 overflow-y-auto">
                    <button class="flex flex-row items-center bg-indigo-300 rounded-xl p-2">
                        <div class="flex items-center justify-center h-8 w-8 bg-indigo-200 rounded-full">
                            P
                        </div>
                        <div class="ml-2 text-sm font-semibold">"Mines of Phandalin 5e"</div>
                    </button>
                    <button class="flex flex-row items-center hover:bg-gray-100 rounded-xl p-2">
                        <div class="flex items-center justify-center h-8 w-8 bg-gray-200 rounded-full">
                            C
                        </div>
                        <div class="ml-2 text-sm font-semibold">Curse of Strahd</div>
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ChatInput(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-row items-center h-16 rounded-xl bg-white w-full px-4">
            <div>
                <button class="flex items-center justify-center text-gray-400 hover:text-gray-600">
                    <svg
                        class="w-5 h-5"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13"
                        ></path>
                    </svg>
                </button>
            </div>
            <div class="flex-grow ml-4">
                <form
                    class="relative w-full m-0"
                    id="chat-input"
                    ws-send
                    hx-on="htmx:wsAfterSend: this.reset();"
                >
                    <input
                        type="text"
                        name="message"
                        class="flex w-full border rounded-xl focus:outline-none focus:border-indigo-300 pl-4 h-10"
                    />
                </form>
            </div>
            <div class="ml-4">
                <button
                    type="submit"
                    form="chat-input"
                    class="flex items-center justify-center bg-indigo-500 hover:bg-indigo-600 rounded-xl text-white px-4 py-1 flex-shrink-0"
                >
                    <span>Send</span>
                    <span class="ml-2">
                        <svg
                            class="w-4 h-4 transform rotate-45 -mt-px"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"
                            ></path>
                        </svg>
                    </span>
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn AiChatMessage(cx: Scope, id: String) -> impl IntoView {
    view! { cx,
        <div class="col-start-1 col-end-8 p-3 rounded-lg">
            <div class="flex flex-row items-center">
                <div class="flex items-center justify-center h-10 w-10 rounded-full bg-indigo-500 flex-shrink-0">
                    Ai
                </div>
                <div class="relative ml-3 text-sm bg-white py-2 px-4 shadow rounded-xl">
                    <div id=format!("ai-msg-{}", id)></div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn UserChatMessage(cx: Scope, msg: String) -> impl IntoView {
    view! { cx,
        <div class="col-start-6 col-end-13 p-3 rounded-lg">
            <div class="flex items-center justify-start flex-row-reverse">
                <div class="flex items-center justify-center h-10 w-10 rounded-full bg-indigo-500 flex-shrink-0">
                    E
                </div>
                <div class="relative mr-3 text-sm bg-indigo-100 py-2 px-4 shadow rounded-xl">
                    <div>{msg}</div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Chat(cx: Scope) -> impl IntoView {
    view! { cx,
        <div hx-ext="ws" ws-connect="/ws" class="flex flex-col flex-auto h-full p-6">
            <div class="flex flex-col flex-auto flex-shrink-0 rounded-2xl bg-gray-100 h-full p-4">
                <div class="flex flex-col h-full overflow-x-auto mb-4">
                    <div class="flex flex-col h-full">
                        <div id="chat-history" class="grid grid-cols-12 gap-y-2"></div>
                    </div>
                </div>
                <ChatInput/>
            </div>
        </div>
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsChatPayload {
    pub message: String,
    #[serde(rename = "HEADERS")]
    headers: Headers,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Headers {
    #[serde(rename = "HX-Request")]
    hx_request: String,
    #[serde(rename = "HX-Trigger")]
    hx_trigger: String,
    #[serde(rename = "HX-Trigger-Name")]
    hx_trigger_name: Option<String>,
    #[serde(rename = "HX-Target")]
    hx_target: String,
    #[serde(rename = "HX-Current-URL")]
    hx_current_url: String,
}

pub fn parse_payload(json_str: &str) -> Result<WsChatPayload, serde_json::Error> {
    serde_json::from_str(json_str)
}
