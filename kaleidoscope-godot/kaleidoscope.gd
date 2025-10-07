extends Sprite2D

@onready var mat := material as ShaderMaterial
var n := 6.0

func _ready():
	# Load the texture resource from your project
	var img_tex: Texture2D = load("res://icon.svg")
	texture = img_tex  # sets the Sprite2D’s visible texture
	
	# Pass it to the shader uniform
	mat.set_shader_parameter("u_tex", img_tex)

func _process(_delta: float) -> void:
	var t = Time.get_ticks_msec() / 1000.0

	# Update uniforms like you did in JS
	mat.set_shader_parameter("out_size", Vector2(2.0, 2.0))
	mat.set_shader_parameter("in_size", Vector2(1.0, 1.0))
	mat.set_shader_parameter("center_in", Vector2(0.5, 0.5))
	mat.set_shader_parameter("center_out", Vector2(1.0, 1.0))
	mat.set_shader_parameter("width", PI / n)
	mat.set_shader_parameter("base_scale", 8.0)
	mat.set_shader_parameter("u_time", t)

	var angle = fmod((t / 15.0) * TAU, TAU)
	mat.set_shader_parameter("r_start", angle)
	mat.set_shader_parameter("r_out", 0.0)
