import {AfterViewInit, Component, ElementRef, EventEmitter, OnInit, Output, ViewChild} from '@angular/core';
import {ActivatedRoute} from "@angular/router";
import {ChatService} from "../../service/chat.service";
import {HttpClient} from "@angular/common/http";
import {MessageService} from "../../service/message.service";
import {ChatMessage} from "../../data/chat-message";
import {NotificationService} from "../../service/notification.service";
import {ChatEntry} from "../../data/chat-entry";
import {UtilService} from "../../service/util.service";

@Component({
  selector: 'app-active-chat',
  templateUrl: './active-chat.component.html',
  styleUrls: ['./active-chat.component.css']
})
export class ActiveChatComponent implements OnInit, AfterViewInit {

  activeChatId!: number

  public activeChat?: ChatEntry

  @ViewChild("message_box")
  messageBox!: ElementRef<HTMLDivElement>

  public messages: ChatMessage[] = []

  constructor(
    private route: ActivatedRoute,
    private chatService: ChatService,
    private messageService: MessageService,
    private notificationService: NotificationService,
    private utilService: UtilService,
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

              let index = this.chatService.chatList.findIndex((c) => c.id == this.activeChatId)
              if (index != -1)
                this.activeChat = this.chatService.chatList[index]
            }
          })

      }
    })
  }

  public backClicked() {
    this.utilService.backButtonClicked.next(0)
  }

  sendMessage(message: string) {
    this.messageService.sendMessage(this.activeChatId, message)
  }

  ngAfterViewInit(): void {
    let self = this;
    this.notificationService.notificationSubject.subscribe({
      next: msg => {

        if (msg.chat_id == this.activeChatId)
          self.messages.push(msg)

      }
    })
  }
}
