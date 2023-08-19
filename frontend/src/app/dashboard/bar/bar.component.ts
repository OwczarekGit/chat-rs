import { Component } from '@angular/core';
import {LoginService} from "../../service/login.service";

@Component({
  selector: 'app-bar',
  templateUrl: './bar.component.html',
  styleUrls: ['./bar.component.css']
})
export class BarComponent {
  constructor(private loginService: LoginService) {
  }

  public logout() {
    this.loginService.logout()
  }

}
