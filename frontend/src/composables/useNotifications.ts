import Notification from "@/components/notifications/Notification.vue";

import { createApp, h, Transition } from "vue";

const NOTIFICATION_CONTAINER_ID = "notifications-container";

const createContainer = () => {
  const container = document.getElementById(NOTIFICATION_CONTAINER_ID);

  if (container) return container;

  const notificationsContainer = document.createElement("div");

  notificationsContainer.id = NOTIFICATION_CONTAINER_ID;
  notificationsContainer.className =
    "fixed right-6 bottom-6 w-[320px] flex flex-col gap-6 items-end pointer-events-none";

  document.body.appendChild(notificationsContainer);

  return notificationsContainer;
};

const showNotification = (
  message: string,
  variant: "warn" | "error" | "success",
  duration = 3000
) => {
  const container = createContainer();
  const notificationContainer = document.createElement("div");
  container.appendChild(notificationContainer);

  const app = createApp({
    render() {
      return h(
        Transition,
        {
          enterFromClass: "opacity-0 translate-x-full",
          leaveToClass: "opacity-0 translate-x-full",
          appear: true,
        },
        [h(Notification, { variant: variant }, () => message)]
      );
    },
  });

  app.mount(notificationContainer);

  setTimeout(() => {
    app.unmount();
    container.removeChild(notificationContainer);
  }, duration);
};

const success = (message: string, duration?: number) =>
  showNotification(message, "success", duration);
const warn = (message: string, duration?: number) =>
  showNotification(message, "warn", duration);
const error = (message: string, duration?: number) =>
  showNotification(message, "error", duration);

export function useNotifications() {
  return {
    success,
    warn,
    error,
  };
}
