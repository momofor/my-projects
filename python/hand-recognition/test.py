import dbus

bus = dbus.SessionBus()

player = dbus.SessionBus().get_object(
    "org.mpris.MediaPlayer2.mpd", "/org/mpris/MediaPlayer2"
)
interface = dbus.Interface(player, dbus_interface="org.mpris.MediaPlayer2.Player")
# interface.PlayPause()

property_interface = dbus.Interface(
    player, dbus_interface="org.freedesktop.DBus.Properties"
)
property_interface.Set("org.mpris.MediaPlayer2.Player", "Position", 91000000)
print(property_interface)
