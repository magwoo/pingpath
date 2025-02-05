import Notification from "@/components/notifications/Notification.vue";
import { createApp, h, Transition } from "vue";

const createContainer = () => {
  let container = document.getElementById("notifications-container");

  if (container) return container;

  let notificationsContainer = document.createElement("div");

  notificationsContainer.id = "notifications-container";
  notificationsContainer.className =
    "fixed right-6 bottom-6 w-[320px] flex flex-col gap-6 justify-end items-end";

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
          onAfterLeave: () => {
            app.unmount();
            document.body.removeChild(container);
          },
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
