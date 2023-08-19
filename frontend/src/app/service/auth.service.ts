import { Injectable } from '@angular/core';
import {CookieService} from "ngx-cookie-service";

@Injectable({
  providedIn: 'root'
})
export class AuthService {

  constructor(private cookies: CookieService) { }

  public isLoggedIn(): boolean {
    return this.cookies.check("AUTH")
  }

  public isNotLoggedIn(): boolean {
    return !this.cookies.check("AUTH")
  }
}
