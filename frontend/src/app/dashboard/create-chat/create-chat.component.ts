import { Component } from '@angular/core';
import {FormControl, FormGroup, Validators} from "@angular/forms";

@Component({
  selector: 'app-create-chat',
  templateUrl: './create-chat.component.html',
  styleUrls: ['./create-chat.component.css']
})
export class CreateChatComponent {

  public form = new FormGroup({
    name: new FormControl<string>('', Validators.required)
  })

  public createChat() {
    let form = this.form.getRawValue()
  }
}
