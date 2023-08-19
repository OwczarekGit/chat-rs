import { Component } from '@angular/core';
import {ChatService} from "../../service/chat.service";
import {ChatEntry} from "../../data/chat-entry";
import {Router} from "@angular/router";

@Component({
  selector: 'app-chat-list',
  templateUrl: './chat-list.component.html',
  styleUrls: ['./chat-list.component.css']
})
export class ChatListComponent {
  constructor(public chatService: ChatService, private router: Router) {
  }

  selectChat($event: ChatEntry) {
    this.router.navigate(['dashboard', 'chat', $event.id])
  }
}
