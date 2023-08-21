import { Injectable } from '@angular/core';
import {Subject} from "rxjs";
import {ChatMessage} from "../data/chat-message";
import {SoundService} from "./sound.service";
import {ProfileService} from "./profile.service";
import {AppNotification} from "../data/app-notification";
import {NotificationType} from "../data/notification-type";
import {ChatService} from "./chat.service";

@Injectable({
  providedIn: 'root'
})
export class NotificationService {

  private es: EventSource;

  public chatMessageSubject: Subject<ChatMessage> = new Subject<ChatMessage>()

  constructor(private soundService: SoundService, private profileService: ProfileService, private chatService: ChatService) {
    this.es = new EventSource("/api/notification/subscribe")

    this.es.addEventListener("message", (msg) => {

      let data: AppNotification = JSON.parse(msg.data)

      switch (data.notification_type) {
        case NotificationType.CHAT_MESSAGE: {
          let body = data.body as ChatMessage

          if (this.profileService.currentUser?.id != body.author_id) {
            this.soundService.playNotificationSound()
          }

          this.chatMessageSubject.next(body)
        } break;
        case NotificationType.CHAT_INVITATION: {
          this.chatService.updateChatList()
        } break;
      }

    })
  }
}
