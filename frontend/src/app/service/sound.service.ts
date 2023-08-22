import { Injectable } from '@angular/core';
import {ProfileService} from "./profile.service";

@Injectable({
  providedIn: 'root'
})
export class SoundService {

  constructor() { }

  public playNotificationSound() {
    let el = document.createElement("audio")
    el.volume = .8
    el.src = "assets/notify.mp3"
    try {
      el.play()
    } catch (err) {}
  }
}
