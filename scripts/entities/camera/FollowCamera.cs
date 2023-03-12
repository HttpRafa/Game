using Godot;

namespace Game.scripts.entities.camera;

public partial class FollowCamera : Camera3D
{

	[Export] private Node3D _target;

	[Export] private Vector3 _offset = new(15f, 15f, 15f);
	[Export] private float _smoothSpeed = 7.5f;

	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
	}

	public override void _PhysicsProcess(double delta)
	{
		Position = Position.Lerp(_target.Position + _offset, _smoothSpeed * (float)delta);
		LookAt(_target.Position);
	}
	
}