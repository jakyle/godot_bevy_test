extends Button

onready var game_scene = preload("res://scenes/Game.tscn").instance()

func _ready():
	pass


func _on_StartGame_button_up():
	var root = get_tree().get_root();
	var current_scene = self.get_parent()
	root.add_child(game_scene)
	current_scene.queue_free()
	root.remove_child(current_scene)
