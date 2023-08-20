import { Component } from '@angular/core';
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {ChatService} from "../../service/chat.service";

@Component({
  selector: 'app-create-chat',
  templateUrl: './create-chat.component.html',
  styleUrls: ['./create-chat.component.css']
})
export class CreateChatComponent {

  public form = new FormGroup({
    name: new FormControl<string>('', Validators.required)
  })

  constructor(private chatService: ChatService) {
  }

  public createChat() {
    let form = this.form.getRawValue()

    this.chatService.createChat(form.name as string)

  }
}
