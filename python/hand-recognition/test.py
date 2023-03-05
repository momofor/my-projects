import dbus

bus = dbus.SessionBus()

player = dbus.SessionBus().get_object(
    "org.mpris.MediaPlayer2.mpd", "/org/mpris/MediaPlayer2"
)
interface = dbus.Interface(player, dbus_interface="org.mpris.MediaPlayer2.Player")
interface.PlayPause()

property_interface = dbus.Interface(
    player, dbus_interface="org.freedesktop.DBus.Properties"
)
volume = property_interface.Get("org.mpris.MediaPlayer2.Player", "Volume")
print(volume)
