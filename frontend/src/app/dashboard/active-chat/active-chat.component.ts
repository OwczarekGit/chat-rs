import {Component, OnInit} from '@angular/core';
import {ActivatedRoute} from "@angular/router";
import {ChatService} from "../../service/chat.service";
import {HttpClient} from "@angular/common/http";
import {MessageService} from "../../service/message.service";

@Component({
  selector: 'app-active-chat',
  templateUrl: './active-chat.component.html',
  styleUrls: ['./active-chat.component.css']
})
export class ActiveChatComponent implements OnInit {

  private activeChatId!: number;

  constructor(
    private route: ActivatedRoute,
    private chatService: ChatService,
    private http: HttpClient,
    private messageService: MessageService,
  ) {
  }

  ngOnInit() {
    this.route.params.subscribe({
      next: value => {
        this.activeChatId = +value['id']
      }
    })
  }

  sendMessage(message: string) {
    this.messageService.sendMessage(this.activeChatId, message)
  }
}
