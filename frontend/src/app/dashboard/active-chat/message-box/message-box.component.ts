import {AfterViewInit, Component, ElementRef, Input, OnChanges, OnInit, SimpleChanges} from '@angular/core';
import {ChatMessage} from "../../../data/chat-message";
import {NotificationService} from "../../../service/notification.service";

@Component({
  selector: 'app-message-box',
  templateUrl: './message-box.component.html',
  styleUrls: ['./message-box.component.css']
})
export class MessageBoxComponent {
  @Input()
  set messages(messages: ChatMessage[]) {
    this._messages = messages
    this.scrollToBottom()
  }

  _messages: ChatMessage[] = []

  constructor(
    private host: ElementRef<HTMLDivElement>,
    private notificationService: NotificationService,
  ) {
    this.notificationService.notificationSubject.subscribe({
      next: v => {
        this.scrollToBottom()
      }
    })
  }

  public track(index: number, msg: ChatMessage) {
    return msg.id
  }

  public scrollToBottom() {
    setTimeout(() => {
      this.host.nativeElement.scrollTop = this.host.nativeElement.scrollHeight + 100
    },0)
  }
}
