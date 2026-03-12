use leptos::{ev::SubmitEvent, logging, prelude::*};

#[component]
pub fn LoginForm() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        logging::log!("Email: {}", email.read());
        logging::log!("Password: {}", password.read());
    };

    view! {
        <div
            class = "w-auto h-auto flex items-center justify-center bg-gray-100"
        >
            <div
                class = "bg-white p-8 rounded-xl shadow-md w-96"
            >
                <h1
                    class = "text-2xl font-bold text-center mb-6"
                >
                    "Login Kost Management"
                </h1>
                <form
                    class = "flex flex-col gap-4"
                    on:submit = on_submit
                >
                    <div>
                        <label
                            class = "block text-sm font-medium mb-1"
                        >
                            "Email"
                        </label>
                        <input 
                            type = "email"
                            placeholder = "admin@gmail.com"
                            on:input:target = move |ev| {
                                set_email.set(ev.target().value());
                            }
                            class = "w-full border rounded-lg px-3 py-2 focus:outline-none"
                        />
                    </div>
                    <div>
                        <label
                            class = "block text-sm font-medium mb-1"
                        >
                            "Password"
                        </label>
                        <input 
                            type = "password"
                            placeholder = "******"
                            on:input:target = move |ev| {
                                set_password.set(ev.target().value());
                            }
                            class = "w-full border rounded-lg px-3 py-2 focus:outline-none"
                        />
                    </div>
                    <button
                        type = "submit"
                        value = "Submit"
                        class = "bg-gray-500 text-white py-2 rounded-lg hover:bg-gray-700 transition"
                    >
                        "Login"
                    </button>
                </form>
            </div>
        </div>
    }
}