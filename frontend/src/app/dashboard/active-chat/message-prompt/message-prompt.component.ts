import {Component, ElementRef, EventEmitter, Output, ViewChild} from '@angular/core';

@Component({
  selector: 'app-message-prompt',
  templateUrl: './message-prompt.component.html',
  styleUrls: ['./message-prompt.component.css']
})
export class MessagePromptComponent {

  message: string = ""

  @Output()
  messageSend: EventEmitter<string> = new EventEmitter<string>()

  sendMessage() {
    if (this.message == "") return;

    this.messageSend.emit(this.message)
    this.message = ""
  }
}
