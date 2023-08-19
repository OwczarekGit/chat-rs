import { Component } from '@angular/core';
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {LoginService} from "../service/login.service";

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css']
})
export class LoginComponent {

  public form = new FormGroup({
    email: new FormControl<string>('', [Validators.required]),
    password: new FormControl<string>('', [Validators.required])
  })

  constructor(private loginService: LoginService) {
  }

  public login() {
    let form = this.form.getRawValue()

    this.loginService.login(
      form.email as string,
      form.password as string,
    )
  }
}
