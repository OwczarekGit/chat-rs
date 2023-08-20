import { Injectable } from '@angular/core';
import {BehaviorSubject, Subject} from "rxjs";
import {ChatMessage} from "../data/chat-message";
import {SoundService} from "./sound.service";
import {ProfileService} from "./profile.service";

@Injectable({
  providedIn: 'root'
})
export class NotificationService {

  private es: EventSource;

  public notificationSubject: Subject<ChatMessage> = new Subject<ChatMessage>()

  constructor(private soundService: SoundService, private profileService: ProfileService) {
    this.es = new EventSource("/api/notification/subscribe")

    this.es.addEventListener("message", (msg) => {

      let data: ChatMessage = JSON.parse(msg.data)

      if (this.profileService.currentUser?.id != data.author_id) {
        this.soundService.playNotificationSound()
      }

      this.notificationSubject.next(data)
    })
  }
}
