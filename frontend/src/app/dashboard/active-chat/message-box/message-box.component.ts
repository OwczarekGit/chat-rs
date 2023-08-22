import {
  AfterViewInit,
  Component,
  ElementRef,
  EventEmitter,
  Input,
  OnChanges,
  OnInit,
  Output,
  SimpleChanges, ViewChild
} from '@angular/core';
import {ChatMessage} from "../../../data/chat-message";
import {NotificationService} from "../../../service/notification.service";

@Component({
  selector: 'app-message-box',
  templateUrl: './message-box.component.html',
  styleUrls: ['./message-box.component.css']
})
export class MessageBoxComponent implements AfterViewInit {

  @ViewChild('top')
  top!: ElementRef<HTMLDivElement>

  @Output()
  reachedTop: EventEmitter<any> = new EventEmitter<any>()

  @Input()
  set messages(messages: ChatMessage[]) {
    this._messages = messages
    setTimeout(() => this.scrollToBottom(),100)
  }

  _messages: ChatMessage[] = []

  constructor(
    private host: ElementRef<HTMLDivElement>,
    private notificationService: NotificationService,
  ) {

    this.notificationService.chatMessageSubject.subscribe({
      next: v => {
        this.scrollToBottom()
      }
    })
  }

  ngAfterViewInit(): void {
    let observer = new IntersectionObserver((e) => {
      this.reachedTop.emit()
    });

    observer.observe(this.top.nativeElement)
  }

  public track(index: number, msg: ChatMessage) {
    return msg.id
  }

  public scrollToBottom() {
    this.host.nativeElement.scrollTop = this.host.nativeElement.scrollHeight
  }
}
