use leptos::prelude::*;

use crate::components::sidebar::Sidebar;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="grid h-full w-full grid-rows-1 overflow-hidden">
            <div class="grid grid-cols-[14rem_1fr] overflow-hidden">
                <Sidebar />
                <main class="grid grid-rows-1 overflow-hidden">

                    <div class="flex flex-col items-center justify-center h-screen  text-text-bright px-6 py-16 bg-background-bright">
                        <svg
                            width="120"
                            height="120"
                            viewBox="0 0 120 120"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                            class="mb-8 animate-pulse"
                        >
                            <rect
                                x="12"
                                y="12"
                                width="96"
                                height="96"
                                rx="5"
                                fill="#1A1B1F"
                                stroke="currentColor"
                                stroke-width="4"
                            />
                            <path
                                d="M40 60 L80 60"
                                stroke="white"
                                stroke-width="6"
                                stroke-linecap="round"
                            />
                            <circle cx="50" cy="50" r="6" fill="white" />
                            <circle cx="70" cy="50" r="6" fill="white" />
                            <path
                                d="M50 75 Q60 85 70 75"
                                stroke="currentColor"
                                stroke-width="4"
                                fill="none"
                            />
                        </svg>
                        <h1 class="text-base font-bold text-primary mb-2 text-shadow-custom">
                            404
                        </h1>
                        <h2 class="text-sm mb-4 text-text-bright text-shadow-custom">
                            Page Not Found
                        </h2>
                        <p class="text-sm text-text-dimmed mb-8 max-w-md text-center">
                            Oops! That page might not exist.<br />
                            But you found our secret neon smiley instead.
                        </p>
                        <a
                            href="/"
                            class="inline-block px-6 py-3 rounded-none bg-primary text-background-bright  shadow-glow-primary hover:bg-apple-400 focus-custom transition"
                        >
                            Go Home
                        </a>
                    </div>
                </main>
            </div>
        </div>
    }
}
