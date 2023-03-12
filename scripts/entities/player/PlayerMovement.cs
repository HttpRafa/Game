using Godot;

namespace Game.scripts.entities.player;

public partial class PlayerMovement : CharacterBody3D
{

	[Export] private float _gravity = ProjectSettings.GetSetting("physics/3d/default_gravity").AsSingle();
	[Export] private float _speed = 5.0f;
	[Export] private float _jumpVelocity = 4.5f;

	[Export] private AimCrossHair _aimCrossHair;

	private bool _isUsingMouse = true;
	
	public override void _PhysicsProcess(double delta)
	{
		Vector3 velocity = Velocity;

		if (_isUsingMouse && (Input.IsActionPressed("look_up") || Input.IsActionPressed("look_down") || Input.IsActionPressed("look_right") || Input.IsActionPressed("look_left")))
		{
			_isUsingMouse = false;
		} else if (!_isUsingMouse && (Input.GetLastMouseVelocity() > Vector2.Zero))
		{
			_isUsingMouse = true;
		}
		
		// Add the gravity
		if (!IsOnFloor())
			velocity.Y -= _gravity * (float)delta;

		// Handle Jump
		if (Input.IsActionJustPressed("jump") && IsOnFloor())
			velocity.Y = _jumpVelocity;

		// Get the input direction and handle the movement/deceleration
		// As good practice, you should replace UI actions with custom gameplay actions
		Vector2 inputDir = Input.GetVector("move_left", "move_right", "move_forward", "move_back");
		Vector3 direction = (Transform.Basis * new Vector3(inputDir.X, 0, inputDir.Y)).Normalized();
		if (direction != Vector3.Zero)
		{
			velocity.X = direction.X * _speed;
			velocity.Z = direction.Z * _speed;
		}
		else
		{
			velocity.X = Mathf.MoveToward(Velocity.X, 0, _speed);
			velocity.Z = Mathf.MoveToward(Velocity.Z, 0, _speed);
		}

		Velocity = velocity;
		MoveAndSlide();
	}
	
}