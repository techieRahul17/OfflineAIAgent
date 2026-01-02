import { invoke } from "@tauri-apps/api/core";

export const askAgent = (context, input) =>
  invoke("ask_agent", { context, input });

export const trackEvent = (event) =>
  invoke("track_event", { event });

export const getFeedback = () =>
  invoke("get_feedback");
