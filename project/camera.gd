extends Camera

export var speed_base = 10
export var speed_mod = 5.0
export var sensitivity_h = 1.0
export var sensitivity_v = 1.0

var paused := false
var speed_current: float

func _ready():
	Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)
	pass

func _input(event):
	if event is InputEventMouseMotion and !paused:
		var angle_x = -event.relative.y * sensitivity_v * 0.002
		angle_x = clamp(angle_x, -PI*0.5-rotation.x, PI*0.5-rotation.x)
		rotate_object_local(Vector3(1,0,0), angle_x)
		var angle_y = -event.relative.x * sensitivity_h * 0.002
		rotate_y(angle_y)

func _physics_process(delta):
	speed_current = speed_base
	if Input.is_key_pressed(KEY_SHIFT):
		speed_current *= speed_mod

	if Input.is_key_pressed(KEY_T):
		translation = Vector3(2, 5, 2)
		rotation_degrees = Vector3(-90, -90, 0)

	if Input.is_action_just_pressed("esc"):
		paused = !paused
		if paused:
			Input.set_mouse_mode(Input.MOUSE_MODE_VISIBLE)
		else:
			Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)

	if paused:
		return

	var dir = Vector3()
	if Input.is_key_pressed(KEY_W):
		dir += Vector3(0,0,-1)
	if Input.is_key_pressed(KEY_S):
		dir += Vector3(0,0,1)
	if Input.is_key_pressed(KEY_A):
		dir += Vector3(-1,0,0)
	if Input.is_key_pressed(KEY_D):
		dir += Vector3(1,0,0)
	if Input.is_key_pressed(KEY_Q):
		dir += Vector3(0,-1,0)
	if Input.is_key_pressed(KEY_E):
		dir += Vector3(0,1,0)

	var vel = dir.normalized() * speed_current

	translate(vel*delta)
