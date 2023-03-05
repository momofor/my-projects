import cv2 
import mediapipe as mp
import dbus
import re
import time
bus = dbus.SessionBus()

player = dbus.SessionBus().get_object(
    "org.mpris.MediaPlayer2.mpd", "/org/mpris/MediaPlayer2"
)
interface = dbus.Interface(player, dbus_interface="org.mpris.MediaPlayer2.Player")
property_interface = dbus.Interface(player, dbus_interface='org.freedesktop.DBus.Properties')

def clamp(n, minn, maxn):
    return max(min(maxn, n), minn)

prev_cnt = 0

def count_fingers(lst):
    cnt = 0

    thresh = (lst.landmark[0].y*100 - lst.landmark[9].y*100)/2

    if (lst.landmark[5].y*100 - lst.landmark[8].y*100) > thresh:
        cnt += 1

    if (lst.landmark[9].y*100 - lst.landmark[12].y*100) > thresh:
        cnt += 1

    if (lst.landmark[13].y*100 - lst.landmark[16].y*100) > thresh:
        cnt += 1

    if (lst.landmark[17].y*100 - lst.landmark[20].y*100) > thresh:
        cnt += 1

    if (lst.landmark[5].x*100 - lst.landmark[4].x*100) > 6:
        cnt += 1


    return cnt 

cap = cv2.VideoCapture(0)

drawing = mp.solutions.drawing_utils
hands = mp.solutions.hands
hand_obj = hands.Hands(max_num_hands=1)


start_init = False 

prev = -1
volume = property_interface.Get("org.mpris.MediaPlayer2.Player", "Volume")

while True:
    end_time = time.time()
    _, frm = cap.read()
    frm = cv2.flip(frm, 1)

    res = hand_obj.process(cv2.cvtColor(frm, cv2.COLOR_BGR2RGB))

    if res.multi_hand_landmarks:

        hand_keyPoints = res.multi_hand_landmarks[0]

        cnt = count_fingers(hand_keyPoints)

        if not(prev==cnt):
            if not(start_init):
                start_time = time.time()
                start_init = True

            elif (end_time-start_time) > 0.2:
                if (cnt == 1):
                    interface.Next()
                
                elif (cnt == 2):
                    interface.Previous()

                elif (cnt == 3):
                    volume = clamp(volume-0.1,0,1)
                    property_interface.Set('org.mpris.MediaPlayer2.Player', 'Volume', volume)
                    print(volume)


                elif (cnt == 4):
                    volume = clamp(volume + 0.1,0,1)  
                    property_interface.Set('org.mpris.MediaPlayer2.Player', 'Volume', volume)
                    print(volume)

                elif (cnt == 5):
                    interface.PlayPause()

                prev = cnt
                start_init = False



        drawing.draw_landmarks(frm, hand_keyPoints, hands.HAND_CONNECTIONS)
        if cnt != prev_cnt: 
            prev_cnt = cnt
            print("I see ", cnt, " fingers")

    cv2.imshow("window", frm)

    if cv2.waitKey(1) == 27:
        cv2.destroyAllWindows()
        cap.release()
        break
