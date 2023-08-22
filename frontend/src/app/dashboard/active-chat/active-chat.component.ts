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

  public offset: number
  public count: number
  public readonly pageSize: number
  private fetchLock: boolean = false
  private fetchedAll: boolean = false

  constructor(
    private route: ActivatedRoute,
    private chatService: ChatService,
    private messageService: MessageService,
    private notificationService: NotificationService,
    private utilService: UtilService,
  ) {
    this.offset = 0
    this.count = 20
    this.pageSize = 20
  }

  ngOnInit() {
    this.route.params.subscribe({
      next: value => {
        this.activeChatId = +value['id']
        this.offset = 0
        this.fetchLock = false
        this.fetchedAll = false
        this.messages = []

        this.fetchNextPage()

        let index = this.chatService.chatList.findIndex((c) => c.id == this.activeChatId)
        if (index != -1)
          this.activeChat = this.chatService.chatList[index]
      }
    })
  }

  public fetchNextPage() {
    if (this.fetchLock || this.fetchedAll) {
      return
    }

    this.fetchLock = true
    this.messageService.getMessagesPaginated(this.activeChatId, this.count, this.offset)
      .subscribe({
        next: msg => {
          this.messages.unshift(...msg)

          const fetchedCount = msg.length

          if (fetchedCount == 0) {
            this.fetchedAll = true
          }

          this.offset += fetchedCount
          this.fetchLock = false
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
    this.notificationService.chatMessageSubject.subscribe({
      next: msg => {

        if (msg.chat_id == this.activeChatId)
          self.messages.push(msg)

      }
    })
  }

  changeChatName(prompt: HTMLInputElement) {
    let text = prompt.value

    if (text == "") return

    this.chatService.changeChatName(this.activeChatId, text)
  }
}
