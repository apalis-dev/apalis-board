use leptos::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Capability {
    // Apalis capabilities
    AcknowledgeService,
    ReadinessService,
    TrackerService,
    Trace,

    // Common Tower services
    Buffer,
    Limit,
    RateLimit,
    Retry,
    Timeout,
    LoadShed,
    Concurrency,
    Balance,
    Discover,
    Reconnect,
    Hedge,
    Filter,

    // Add other capabilities as needed
    Other(String),
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Capability::Other(s) => write!(f, "{s}"),
            _ => write!(f, "{self:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceInfo {
    pub capabilities: Vec<Capability>,
    pub fn_name: Option<String>,
}

pub struct ServiceTypeParser;

impl ServiceTypeParser {
    pub fn parse(type_str: &str) -> Result<ServiceInfo, String> {
        let normalized = type_str.replace(" ", "");
        let mut capabilities = Vec::new();
        let mut fn_name = None;

        Self::extract_info(&normalized, &mut capabilities, &mut fn_name)?;

        Ok(ServiceInfo {
            capabilities,
            fn_name,
        })
    }

    fn extract_info(
        s: &str,
        capabilities: &mut Vec<Capability>,
        fn_name: &mut Option<String>,
    ) -> Result<(), String> {
        let mut pos = 0;

        while pos < s.len() {
            // Find the next type identifier
            if let Some(type_start) = s[pos..].find(|c: char| c.is_ascii_alphabetic() || c == '_') {
                let type_start = pos + type_start;

                // Find the end of the identifier (before '<' or ',')
                let type_end = s[type_start..]
                    .find(['<', ',', '>'])
                    .map(|i| type_start + i)
                    .unwrap_or(s.len());

                let type_name = &s[type_start..type_end];

                // Extract the simple type name (last component after ::)
                let simple_name = type_name.split("::").last().unwrap_or(type_name);

                // Check if this is a capability we're interested in
                if let Some(cap) = Self::parse_capability(simple_name)
                    && !capabilities.contains(&cap)
                {
                    capabilities.push(cap);
                }

                // Check for TaskFn to extract function name
                if simple_name == "TaskFn"
                    && let Some(fn_pos) = s[type_end..].find('<')
                {
                    let inner_start = type_end + fn_pos + 1;
                    if let Some(fn_extracted) = Self::extract_task_fn_name(&s[inner_start..]) {
                        *fn_name = Some(fn_extracted);
                    }
                }

                pos = type_end;
            } else {
                pos += 1;
            }
        }

        Ok(())
    }

    fn extract_task_fn_name(s: &str) -> Option<String> {
        // TaskFn format: TaskFn<module::function_name, ...>
        // We want to extract the function name (first generic parameter)
        let end = s.find(',')?;
        let full_path = s[..end].trim();

        // Get the last component of the path
        let fn_name = full_path.split("::").last()?;
        Some(fn_name.to_string())
    }

    fn parse_capability(name: &str) -> Option<Capability> {
        match name {
            "AcknowledgeService" => Some(Capability::AcknowledgeService),
            "ReadinessService" => Some(Capability::ReadinessService),
            "TrackerService" => Some(Capability::TrackerService),
            "Trace" => Some(Capability::Trace),
            "Buffer" => Some(Capability::Buffer),
            "Limit" => Some(Capability::Limit),
            "RateLimit" => Some(Capability::RateLimit),
            "Retry" => Some(Capability::Retry),
            "Timeout" => Some(Capability::Timeout),
            "LoadShed" => Some(Capability::LoadShed),
            "ConcurrencyLimit" => Some(Capability::Concurrency),
            "Balance" => Some(Capability::Balance),
            "Discover" => Some(Capability::Discover),
            "Reconnect" => Some(Capability::Reconnect),
            "Hedge" => Some(Capability::Hedge),
            "Filter" => Some(Capability::Filter),
            _ => None,
        }
    }
}
fn capability_icon_svg(capability: Capability) -> impl IntoView {
    match capability {
        // Apalis capabilities
        Capability::AcknowledgeService => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <polyline points="20 6 9 17 4 12"></polyline>
            </svg>
        }.into_any(),
        Capability::ReadinessService => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <circle cx="12" cy="12" r="10"></circle>
                <circle cx="12" cy="12" r="3"></circle>
            </svg>
        }.into_any(),
        Capability::TrackerService => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <line x1="12" y1="20" x2="12" y2="10"></line>
                <line x1="18" y1="20" x2="18" y2="4"></line>
                <line x1="6" y1="20" x2="6" y2="16"></line>
            </svg>
        }.into_any(),
        Capability::Trace => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <circle cx="11" cy="11" r="8"></circle>
                <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
            </svg>
        }.into_any(),
        Capability::Buffer => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>
            </svg>
        }.into_any(),
        Capability::Limit => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
                <path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
            </svg>
        }.into_any(),
        Capability::RateLimit => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <circle cx="12" cy="12" r="10"></circle>
                <polyline points="12 6 12 12 16 14"></polyline>
            </svg>
        }.into_any(),
        Capability::Retry => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <polyline points="23 4 23 10 17 10"></polyline>
                <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"></path>
            </svg>
        }.into_any(),
        Capability::Timeout => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <circle cx="12" cy="13" r="8"></circle>
                <path d="M12 9v4l2 2"></path>
                <path d="M5 3L3 5"></path>
                <path d="M19 3l2 2"></path>
            </svg>
        }.into_any(),
        Capability::LoadShed => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <line x1="12" y1="1" x2="12" y2="23"></line>
                <path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"></path>
            </svg>
        }.into_any(),
        Capability::Concurrency => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon>
            </svg>
        }.into_any(),
        Capability::Balance => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <line x1="12" y1="3" x2="12" y2="21"></line>
                <path d="M8 9l4-4 4 4"></path>
                <path d="M16 15l-4 4-4-4"></path>
            </svg>
        }.into_any(),
        Capability::Discover => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <circle cx="12" cy="12" r="10"></circle>
                <polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76"></polygon>
            </svg>
        }.into_any(),
        Capability::Reconnect => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="M6 9H4.5a2.5 2.5 0 0 1 0-5H6"></path>
                <path d="M18 9h1.5a2.5 2.5 0 0 0 0-5H18"></path>
                <path d="M4 22h16"></path>
                <path d="M10 14.66V17c0 .55-.47.98-.97 1.21C7.85 18.75 7 20.24 7 22"></path>
                <path d="M14 14.66V17c0 .55.47.98.97 1.21C16.15 18.75 17 20.24 17 22"></path>
                <path d="M18 2H6v7a6 6 0 0 0 12 0V2Z"></path>
            </svg>
        }.into_any(),
        Capability::Hedge => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path>
            </svg>
        }.into_any(),
        Capability::Filter => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"></polygon>
            </svg>
        }.into_any(),
        Capability::Other(_) => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <circle cx="12" cy="12" r="3"></circle>
                <path d="M12 1v6m0 6v6"></path>
                <path d="m4.93 4.93 4.24 4.24m5.66 5.66 4.24 4.24"></path>
                <path d="M1 12h6m6 0h6"></path>
                <path d="m4.93 19.07 4.24-4.24m5.66-5.66 4.24-4.24"></path>
            </svg>
        }.into_any(),
    }
}

#[component]
pub fn ServiceInfoDisplay(service: Signal<String>) -> impl IntoView {
    let service_info = Memo::new(move |_| {
        ServiceTypeParser::parse(&service.get()).unwrap_or(ServiceInfo {
            capabilities: vec![],
            fn_name: None,
        })
    });
    view! {
        <div class="flex gap-2 items-center text-charcoal-200">
            {move || {
                service_info
                    .get()
                    .fn_name
                    .map(|name| {
                        view! {
                            <span class="inline-flex items-center gap-2 font-mono text-xs px-2 py-1 rounded-md bg-charcoal-700">
                                <span class="w-5 h-5 flex items-center justify-center rounded bg-charcoal-900 text-white text-xxs font-bold">
                                    "ƒ"
                                </span>
                                {name}
                            </span>
                        }
                    })
            }}
            <div class="flex gap-1">
                {move || {
                    let capabilities = service_info.get().capabilities.clone();
                    capabilities
                        .into_iter()
                        .map(|cap| {
                            let label = cap.to_string();
                            let icon_svg = capability_icon_svg(cap);
                            view! {
                                <div class="flex items-center gap-1 px-1 py-0.5 rounded-sm border border-charcoal-700 bg-charcoal-800 hover:bg-charcoal-700 relative">
                                    <div class="has-tooltip">
                                        <span class="tooltip rounded shadow-sm p-1 bg-charcoal-100 text-charcoal-700 -mt-8">
                                            {label}
                                        </span>
                                        <span class="flex-shrink-0" style:color="white">
                                            {icon_svg}
                                        </span>
                                    </div>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_apalis_service() {
        let type_str = "apalis_core::worker::ext::ack::AcknowledgeService<apalis::layers::tracing::Trace<apalis_core::worker::ReadinessService<apalis_core::worker::TrackerService<apalis_core::task_fn::TaskFn<rest_api::send_email, rest_api::Email, apalis_sqlite::context::SqliteContext, ()>>>>, apalis_sqlite::ack::SqliteAck>";

        let result = ServiceTypeParser::parse(type_str).unwrap();

        assert_eq!(result.fn_name, Some("send_email".to_string()));
        assert!(
            result
                .capabilities
                .contains(&Capability::AcknowledgeService)
        );
        assert!(result.capabilities.contains(&Capability::Trace));
        assert!(result.capabilities.contains(&Capability::ReadinessService));
        assert!(result.capabilities.contains(&Capability::TrackerService));
    }

    #[test]
    fn test_parse_without_task_fn() {
        let type_str =
            "tower::timeout::Timeout<tower::buffer::Buffer<tower::limit::RateLimit<MyService>>>";

        let result = ServiceTypeParser::parse(type_str).unwrap();

        assert_eq!(result.fn_name, None);
        assert!(result.capabilities.contains(&Capability::Timeout));
        assert!(result.capabilities.contains(&Capability::Buffer));
        assert!(result.capabilities.contains(&Capability::RateLimit));
    }

    #[test]
    fn test_parse_simple_service() {
        let type_str = "tower::retry::Retry<tower::timeout::Timeout<MyService>>";

        let result = ServiceTypeParser::parse(type_str).unwrap();

        assert_eq!(result.fn_name, None);
        assert_eq!(result.capabilities.len(), 2);
        assert!(result.capabilities.contains(&Capability::Retry));
        assert!(result.capabilities.contains(&Capability::Timeout));
    }
}
