import {AfterViewInit, Component, ElementRef, OnInit, ViewChild} from '@angular/core';
import {ActivatedRoute} from "@angular/router";
import {ChatService} from "../../service/chat.service";
import {HttpClient} from "@angular/common/http";
import {MessageService} from "../../service/message.service";
import {ChatMessage} from "../../data/chat-message";
import {NotificationService} from "../../service/notification.service";
import {ChatEntry} from "../../data/chat-entry";

@Component({
  selector: 'app-active-chat',
  templateUrl: './active-chat.component.html',
  styleUrls: ['./active-chat.component.css']
})
export class ActiveChatComponent implements OnInit, AfterViewInit {

  private activeChatId!: number;

  @ViewChild("message_box")
  messageBox!: ElementRef<HTMLDivElement>

  public messages: ChatMessage[] = []

  constructor(
    private route: ActivatedRoute,
    private chatService: ChatService,
    private http: HttpClient,
    private messageService: MessageService,
    private notificationService: NotificationService,
  ) {
  }

  ngOnInit() {
    this.route.params.subscribe({
      next: value => {
        this.activeChatId = +value['id']

        this.messageService.getAllMessages(this.activeChatId)
          .subscribe({
            next: msg => {
              this.messages = msg
            }
          })
      }
    })
  }

  public track(index: number, msg: ChatMessage): number {
    return msg.id
  }

  sendMessage(message: string) {
    this.messageService.sendMessage(this.activeChatId, message)
  }

  ngAfterViewInit(): void {
    let self = this;
    this.notificationService.notificationSubject.subscribe({
      next: msg => {
          self.messages.push(msg)
      }
    })
  }
}
