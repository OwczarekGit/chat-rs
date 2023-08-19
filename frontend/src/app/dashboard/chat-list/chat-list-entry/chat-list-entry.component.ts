import {Component, EventEmitter, Input, Output} from '@angular/core';
import {ChatEntry} from "../../../data/chat-entry";

@Component({
  selector: 'app-chat-list-entry',
  templateUrl: './chat-list-entry.component.html',
  styleUrls: ['./chat-list-entry.component.css']
})
export class ChatListEntryComponent {

  @Input()
  public chatEntry!: ChatEntry

  @Output()
  public chatClicked: EventEmitter<ChatEntry> = new EventEmitter<ChatEntry>()

}
