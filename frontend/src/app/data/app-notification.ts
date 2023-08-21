import {NotificationType} from "./notification-type";

export interface AppNotification {
  notification_type: NotificationType,
  body: any
}
