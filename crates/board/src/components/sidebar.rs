use leptos::{ev::MouseEvent, prelude::*};
use leptos_router::{components::A, hooks::use_location};

use crate::use_sse_provider;

#[component]
pub fn Sidebar() -> impl IntoView {
    let version = env!("CARGO_PKG_VERSION");
    let (workflows_expanded, set_workflows_expanded) = signal(true);
    let (manage_expanded, set_manage_expanded) = signal(true);
    let location = use_location();

    let sse = use_sse_provider();

    let is_healthy = move || {
        sse.is_healthy()
            .get()
            .then(|| view! { <span class="text-green-600">"Healthy"</span> })
            .unwrap_or(view! { <span class="text-red-600">"Unhealthy"</span> })
    };

    let status_message = move || {
        if sse.is_healthy()
            .get() { "All systems are operational." } else { "System unhealthy" }
    };

    view! {
        <div class="grid h-full grid-rows-[2.75rem_1fr_auto] overflow-hidden border-r border-grid-bright bg-background-bright transition">
            <div class="flex items-center justify-between overflow-hidden border-b px-3 py-1 transition duration-300 border-grid-bright">
                <div class="flex flex-row text-text-bright font-normal text-center font-sans items-center gap-1">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 512 512"
                        height="32"
                        width="32"
                        class="m-1 text-[white] shrink-0"
                        fill="currentColor"
                    >
                        <polygon points="141.598,307.12 0,448.707 42.972,448.707 174.577,317.114"></polygon>
                        <path d="M511.324,156.078c-1.335-3.15-4.427-5.197-7.848-5.197H459.55c-4.709,0-8.524,3.816-8.524,8.524
                        l12.519,41.258c1.655,1.602,3.793,2.399,5.927,2.399c2.229,0,4.454-0.868,6.126-2.596l34.006-35.133
                        C511.981,162.873,512.659,159.229,511.324,156.078z"></path>
                        <path d="M321.452,365.844c-91.686,0-129.88-64.005-128.392-110.162
                        c-0.011-0.011,192.355-192.389,192.355-192.389c37.778,20.889,67.236,55.007,82.09,96.115c4.069,11.229,7.035,22.98,8.785,35.13
                        c1.227,8.456,1.864,17.093,1.864,25.878c0,2.75-0.057,5.501-0.193,8.217C477.961,228.633,425.246,365.844,321.452,365.844z"></path>
                        <path d="M409.805,228.633h68.157c-4.285,95.285-82.897,171.216-179.24,171.216
                        c-56.542,0-106.969-26.163-139.848-67.032c-6.478-8.024-12.252-16.616-17.275-25.697l51.45-51.45
                        c14.775,44.21,56.508,76.078,105.673,76.078C357.457,331.749,405.577,286.288,409.805,228.633z"></path>
                        <path d="M393.325,197.174c-20.824,0-37.766-16.942-37.766-37.766c0-20.831,16.942-37.778,37.766-37.778
                        c20.831,0,37.778,16.947,37.778,37.778C431.103,180.232,414.156,197.174,393.325,197.174z"></path>
                        <path d="M393.325,144.36c8.308,0,15.047,6.74,15.047,15.047s-6.74,15.036-15.047,15.036
                        s-15.036-6.728-15.036-15.036S385.017,144.36,393.325,144.36z"></path>
                    </svg>
                    <h3 class="font-sans text-sm leading-5 font-medium text-text-bright">
                        "Apalis Board"
                    </h3>
                </div>
            </div>

            // Main Navigation Content
            <div class="overflow-hidden overflow-y-auto pt-2 scrollbar-thin scrollbar-track-transparent scrollbar-thumb-charcoal-600">
                <div class="mb-6 flex flex-col gap-4 px-1">
                    // Main Navigation Items
                    {move || {
                        view! {
                            <div>
                                <NavItem
                                    href="/"
                                    icon=overview_icon()
                                    text="Overview"
                                    is_active=location.pathname.get() == "/"
                                />
                                <NavItem
                                    href="/queues"
                                    icon=queues_icon()
                                    text="Queues"
                                    is_active=location.pathname.get().starts_with("/queues")
                                />
                                <NavItem
                                    href="/tasks"
                                    icon=tasks_icon()
                                    text="Tasks"
                                    is_active=location.pathname.get().starts_with("/tasks")
                                />
                                <NavItem
                                    href="/workers"
                                    icon=workers_icon()
                                    text="Workers"
                                    is_active=location.pathname.get().starts_with("/workers")
                                />
                            </div>
                        }
                    }} // Pro Sections
                    <div>
                        <ExpandableSection
                            title="Advanced"
                            is_expanded=workflows_expanded
                            on_toggle=move |_| set_workflows_expanded.update(|x| *x = !*x)
                        />
                        <Show when=move || workflows_expanded.get()>
                            <div class="opacity-100">
                                <NavItem
                                    href="/pro?interest=stepped"
                                    icon=stepped_icon()
                                    text="Stepped"
                                    badge="Pro".to_string()
                                />
                                <NavItem
                                    href="/pro?interest=complex"
                                    icon=complex_icon()
                                    text="Complex"
                                    badge="Pro".to_string()
                                />
                                <NavItem
                                    href="/pro?interest=schedules"
                                    icon=schedules_icon()
                                    text="Schedules"
                                    badge="Pro".to_string()
                                />
                            </div>
                        </Show>
                    // Manage Section
                    </div> <div>
                        <ExpandableSection
                            title="Manage"
                            is_expanded=manage_expanded
                            on_toggle=move |_| set_manage_expanded.update(|x| *x = !*x)
                        />
                        <Show when=move || manage_expanded.get()>
                            <div class="opacity-100">
                                <NavItem
                                    href="/logs"
                                    icon=logs_icon()
                                    text="Logs"
                                    is_active=location.pathname.get().starts_with("/logs")
                                />
                                <NavItem
                                    href="/settings"
                                    icon=settings_icon()
                                    text="Settings"
                                    is_active=location.pathname.get().starts_with("/settings")
                                />
                            </div>
                        </Show>
                    </div>
                </div>
            </div>

            // Footer
            <div class="font-sans text-xs text-text-dimmed">
                <div class="flex flex-col gap-1 border-t border-grid-bright p-1">
                    <div class="flex flex-col w-full mx-2">
                        <div class="flex gap-2 text-sm">
                            <div class="h-2 w-2 rounded-full bg-green-500 animate-pulse my-[0.3rem]"></div>
                            <span class="text-muted-foreground">"System Status:"</span>
                            {is_healthy}

                        </div>
                        <div class="mt-1 text-xs text-muted-foreground">{status_message}</div>
                    </div>
                </div>

                <div class="flex flex-row gap-1 border-t border-grid-bright p-1 mt-1">
                    <span class="mx-2 text-xs text-muted-foreground border-r border-grid-bright pr-2">
                        "(c) 2025"
                    </span>
                    <span class="mx-2 text-xs text-muted-foreground">{version}</span>
                </div>
            </div>
        </div>
    }
}

#[component]
fn NavItem(
    href: &'static str,
    icon: impl IntoView + 'static,
    text: &'static str,
    #[prop(optional)] is_active: bool,
    #[prop(optional)] badge: Option<String>,
) -> impl IntoView {
    let active_class = if is_active {
        "bg-tertiary text-text-bright"
    } else {
        "bg-transparent group-hover/button:bg-charcoal-750 text-text-bright group-hover:bg-charcoal-750 focus-visible:outline-none focus-visible:ring-0 focus-visible:ring-offset-0 group-hover:text-text-bright"
    };

    view! {
        <A href=href attr:class="group/button focus-custom w-full">
            <div class=format!(
                "flex font-normal text-center font-sans justify-center items-center shrink-0 transition duration-150 rounded-none select-none group-focus/button:outline-none group-disabled/button:opacity-75 group-disabled/button:pointer-events-none focus-custom h-[2rem] px-[0.4rem] text-sm {}",
                active_class,
            )>
                <div class="text-left flex w-full items-center gap-x-1.5">
                    {icon}
                    <div class="flex w-full items-center justify-between">
                        {text}
                        <div class="flex items-center gap-1">
                            {badge
                                .map(|badge_text| {
                                    view! {
                                        <button class="h-fit" tabindex="-1">
                                            <div class="grid place-items-center border border-charcoal-650 rounded-sm px-1 h-4 text-xxs bg-background-bright text-blue-500 whitespace-nowrap">
                                                <span>{badge_text}</span>
                                            </div>
                                        </button>
                                    }
                                })}
                        </div>
                    </div>
                </div>
            </div>
        </A>
    }
}

#[component]
fn ExpandableSection<F>(
    title: &'static str,
    is_expanded: ReadSignal<bool>,
    on_toggle: F,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let chevron_class = move || {
        if is_expanded.get() {
            "transform: none;"
        } else {
            "transform: rotate(-90deg);"
        }
    };

    view! {
        <div
            class="flex cursor-pointer items-center gap-1 py-1 pl-1.5 text-text-dimmed transition hover:bg-charcoal-750 hover:text-text-bright"
            on:click=on_toggle
        >
            <h2 class="text-xs">{title}</h2>
            <div style=chevron_class>
                <svg
                    class="size-2"
                    viewBox="0 0 6 5"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        d="M3.43301 4.25C3.24056 4.58333 2.75944 4.58333 2.56699 4.25L0.834937 1.25C0.642486 0.916667 0.883049 0.5 1.26795 0.5L4.73205 0.5C5.11695 0.5 5.35751 0.916667 5.16506 1.25L3.43301 4.25Z"
                        fill="currentColor"
                    ></path>
                </svg>
            </div>
        </div>
    }
}

// Icon functions
fn overview_icon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
            aria-hidden="true"
            class="h-[1.125rem] shrink-0 justify-start"
        >
            <path d="M14 17h2.75A2.25 2.25 0 0 0 19 14.75v-9.5A2.25 2.25 0 0 0 16.75 3H14v14ZM12.5 3h-5v14h5V3ZM3.25 3H6v14H3.25A2.25 2.25 0 0 1 1 14.75v-9.5A2.25 2.25 0 0 1 3.25 3Z"></path>
        </svg>
    }
}

pub fn queues_icon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
            aria-hidden="true"
            class="h-[1.125rem] text-text-dimmed shrink-0 justify-start"
        >
            <path d="M5.127 3.502 5.25 3.5h9.5c.041 0 .082 0 .123.002A2.251 2.251 0 0 0 12.75 2h-5.5a2.25 2.25 0 0 0-2.123 1.502ZM1 10.25A2.25 2.25 0 0 1 3.25 8h13.5A2.25 2.25 0 0 1 19 10.25v5.5A2.25 2.25 0 0 1 16.75 18H3.25A2.25 2.25 0 0 1 1 15.75v-5.5ZM3.25 6.5c-.04 0-.082 0-.123.002A2.25 2.25 0 0 1 5.25 5h9.5c.98 0 1.814.627 2.123 1.502a3.819 3.819 0 0 0-.123-.002H3.25Z"></path>
        </svg>
    }
}

pub fn tasks_icon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
            aria-hidden="true"
            class="h-[1.125rem] text-text-dimmed shrink-0 justify-start"
        >
            <path
                fill-rule="evenodd"
                d="M7.84 1.804A1 1 0 0 1 8.82 1h2.36a1 1 0 0 1 .98.804l.331 1.652a6.993 6.993 0 0 1 1.929 1.115l1.598-.54a1 1 0 0 1 1.186.447l1.18 2.044a1 1 0 0 1-.205 1.251l-1.267 1.113a7.047 7.047 0 0 1 0 2.228l1.267 1.113a1 1 0 0 1 .206 1.25l-1.18 2.045a1 1 0 0 1-1.187.447l-1.598-.54a6.993 6.993 0 0 1-1.929 1.115l-.33 1.652a1 1 0 0 1-.98.804H8.82a1 1 0 0 1-.98-.804l-.331-1.652a6.993 6.993 0 0 1-1.929-1.115l-1.598.54a1 1 0 0 1-1.186-.447l-1.18-2.044a1 1 0 0 1 .205-1.251l1.267-1.114a7.05 7.05 0 0 1 0-2.227L1.821 7.773a1 1 0 0 1-.206-1.25l1.18-2.045a1 1 0 0 1 1.187-.447l1.598.54A6.992 6.992 0 0 1 7.51 3.456l.33-1.652ZM10 13a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z"
                clip-rule="evenodd"
            ></path>
        </svg>
    }
}

fn schedules_icon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
            aria-hidden="true"
            class="h-[1.125rem] text-text-dimmed shrink-0 justify-start"
        >
            <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 1 0 0-16 8 8 0 0 0 0 16Zm.75-13a.75.75 0 0 0-1.5 0v5c0 .414.336.75.75.75h4a.75.75 0 0 0 0-1.5h-3.25V5Z"
                clip-rule="evenodd"
            ></path>
        </svg>
    }
}

pub fn workers_icon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
            aria-hidden="true"
            class="h-[1.125rem] text-text-dimmed shrink-0 justify-start"
        >
            <path
                fill-rule="evenodd"
                d="M4.25 2A2.25 2.25 0 0 0 2 4.25v2.5A2.25 2.25 0 0 0 4.25 9h2.5A2.25 2.25 0 0 0 9 6.75v-2.5A2.25 2.25 0 0 0 6.75 2h-2.5Zm0 9A2.25 2.25 0 0 0 2 13.25v2.5A2.25 2.25 0 0 0 4.25 18h2.5A2.25 2.25 0 0 0 9 15.75v-2.5A2.25 2.25 0 0 0 6.75 11h-2.5Zm9-9A2.25 2.25 0 0 0 11 4.25v2.5A2.25 2.25 0 0 0 13.25 9h2.5A2.25 2.25 0 0 0 18 6.75v-2.5A2.25 2.25 0 0 0 15.75 2h-2.5Zm0 9A2.25 2.25 0 0 0 11 13.25v2.5A2.25 2.25 0 0 0 13.25 18h2.5A2.25 2.25 0 0 0 18 15.75v-2.5A2.25 2.25 0 0 0 15.75 11h-2.5Z"
                clip-rule="evenodd"
            ></path>
        </svg>
    }
}

fn stepped_icon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
            aria-hidden="true"
            class="h-[1.125rem] text-text-dimmed shrink-0 justify-start"
        >
            <path d="M4.464 3.162A2 2 0 0 1 6.28 2h7.44a2 2 0 0 1 1.816 1.162l1.154 2.5c.067.145.115.291.145.438A3.508 3.508 0 0 0 16 6H4c-.288 0-.568.035-.835.1.03-.147.078-.293.145-.438l1.154-2.5Z"></path>
            <path
                fill-rule="evenodd"
                d="M2 9.5a2 2 0 0 1 2-2h12a2 2 0 1 1 0 4H4a2 2 0 0 1-2-2Zm13.24 0a.75.75 0 0 1 .75-.75H16a.75.75 0 0 1 .75.75v.01a.75.75 0 0 1-.75.75h-.01a.75.75 0 0 1-.75-.75V9.5Zm-2.25-.75a.75.75 0 0 0-.75.75v.01c0 .414.336.75.75.75H13a.75.75 0 0 0 .75-.75V9.5a.75.75 0 0 0-.75-.75h-.01ZM2 15a2 2 0 0 1 2-2h12a2 2 0 1 1 0 4H4a2 2 0 0 1-2-2Zm13.24 0a.75.75 0 0 1 .75-.75H16a.75.75 0 0 1 .75.75v.01a.75.75 0 0 1-.75.75h-.01a.75.75 0 0 1-.75-.75V15Zm-2.25-.75a.75.75 0 0 0-.75.75v.01c0 .414.336.75.75.75H13a.75.75 0 0 0 .75-.75V15a.75.75 0 0 0-.75-.75h-.01Z"
                clip-rule="evenodd"
            ></path>
        </svg>
    }
}

fn complex_icon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
            aria-hidden="true"
            class="h-[1.125rem] text-text-dimmed shrink-0 justify-start"
        >
            <path d="M10.362 1.093a.75.75 0 0 0-.724 0L2.523 5.018 10 9.143l7.477-4.125-7.115-3.925ZM18 6.443l-7.25 4v8.25l6.862-3.786A.75.75 0 0 0 18 14.25V6.443ZM9.25 18.693v-8.25l-7.25-4v7.807a.75.75 0 0 0 .388.657l6.862 3.786Z"></path>
        </svg>
    }
}

pub fn logs_icon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
            aria-hidden="true"
            class="h-[1.125rem] text-text-dimmed shrink-0 justify-start"
        >
            <path d="M4.214 3.227a.75.75 0 0 0-1.156-.955 8.97 8.97 0 0 0-1.856 3.825.75.75 0 0 0 1.466.316 7.47 7.47 0 0 1 1.546-3.186ZM16.942 2.272a.75.75 0 0 0-1.157.955 7.47 7.47 0 0 1 1.547 3.186.75.75 0 0 0 1.466-.316 8.971 8.971 0 0 0-1.856-3.825Z"></path>
            <path
                fill-rule="evenodd"
                d="M10 2a6 6 0 0 0-6 6c0 1.887-.454 3.665-1.257 5.234a.75.75 0 0 0 .515 1.076 32.91 32.91 0 0 0 3.256.508 3.5 3.5 0 0 0 6.972 0 32.903 32.903 0 0 0 3.256-.508.75.75 0 0 0 .515-1.076A11.448 11.448 0 0 1 16 8a6 6 0 0 0-6-6Zm0 14.5a2 2 0 0 1-1.95-1.557 33.54 33.54 0 0 0 3.9 0A2 2 0 0 1 10 16.5Z"
                clip-rule="evenodd"
            ></path>
        </svg>
    }
}

fn settings_icon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
            aria-hidden="true"
            class="h-[1.125rem] text-text-dimmed shrink-0 justify-start"
        >
            <path
                fill-rule="evenodd"
                d="M8.34 1.804A1 1 0 0 1 9.32 1h1.36a1 1 0 0 1 .98.804l.295 1.473c.497.144.971.342 1.416.587l1.25-.834a1 1 0 0 1 1.262.125l.962.962a1 1 0 0 1 .125 1.262l-.834 1.25c.245.445.443.919.587 1.416l1.473.294a1 1 0 0 1 .804.98v1.361a1 1 0 0 1-.804.98l-1.473.295a6.95 6.95 0 0 1-.587 1.416l.834 1.25a1 1 0 0 1-.125 1.262l-.962.962a1 1 0 0 1-1.262.125l-1.25-.834a6.953 6.953 0 0 1-1.416.587l-.294 1.473a1 1 0 0 1-.98.804H9.32a1 1 0 0 1-.98-.804l-.295-1.473a6.957 6.957 0 0 1-1.416-.587l-1.25.834a1 1 0 0 1-1.262-.125l-.962-.962a1 1 0 0 1-.125-1.262l.834-1.25a6.957 6.957 0 0 1-.587-1.416l-1.473-.294A1 1 0 0 1 1 10.68V9.32a1 1 0 0 1 .804-.98l1.473-.295c.144-.497.342-.971.587-1.416l-.834-1.25a1 1 0 0 1 .125-1.262l.962-.962A1 1 0 0 1 5.38 3.03l1.25.834a6.957 6.957 0 0 1 1.416-.587l.294-1.473ZM13 10a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
                clip-rule="evenodd"
            ></path>
        </svg>
    }
}
