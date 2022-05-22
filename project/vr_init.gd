extends Node


var vr_config := preload("res://addons/godot-openxr/config/OpenXRConfig.gdns").new()
var vr_interface: ARVRInterface


func _ready():
	pass


func _on_Debug_pressed():
	$Camera.toggle_pause()
	$StartupMenu.hide()


func _on_VR_pressed():
	vr_interface = ARVRServer.find_interface("OpenXR")
	var inited = vr_interface.initialize()
	if vr_interface and inited:
		OS.vsync_enabled = false
		$VRViewport.arvr = true

	$StartupMenu.hide()
