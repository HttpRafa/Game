using Godot;

namespace Game.scripts.entities.player;

public partial class PlayerMovement : CharacterBody3D
{

	[Export] private float _gravity = ProjectSettings.GetSetting("physics/3d/default_gravity").AsSingle();
	[Export] private float _speed = 6.0f;
	[Export] private float _jumpVelocity = 4.5f;
	[Export] private float _rotationSpeed = 8f;

	[Export] private Crosshair _crosshair;
	
	private bool _isUsingMouse = true;

	private Vector3 _rotationTarget;

	public override void _Ready()
	{
		_crosshair.RotationTargetChanged += target =>
		{
			_rotationTarget = target;
		};
	}

	public override void _PhysicsProcess(double delta)
	{
		if (_isUsingMouse && (Input.IsActionPressed("look_up") || Input.IsActionPressed("look_down") || Input.IsActionPressed("look_right") || Input.IsActionPressed("look_left")))
		{
			_isUsingMouse = false;
			_crosshair.Hide();
		} else if (!_isUsingMouse && (Input.GetLastMouseVelocity() > Vector2.Zero))
		{
			_isUsingMouse = true;
			_crosshair.Show();
		}
		
		MovePlayer(delta);
		RotatePlayer(delta);
	}

	private void RotatePlayer(double delta)
	{
		if (_isUsingMouse)
		{
			Vector3 target = _rotationTarget;
			target.Y = Position.Y;
			Quaternion rotation = Transform.LookingAt(target, Vector3.Up).Basis.GetRotationQuaternion();
			Rotation = Quaternion.FromEuler(Rotation).Slerp(rotation, _rotationSpeed * (float)delta).GetEuler();
		}
		else
		{
			Vector2 gamepadInput = Input.GetVector("look_left", "look_right", "look_up", "look_down");
			Vector3 gamepadLook = new Vector3(gamepadInput.X, 0, gamepadInput.Y) + Position;
			Quaternion rotation = Transform.LookingAt(gamepadLook, Vector3.Up).Basis.GetRotationQuaternion();
			Rotation = Quaternion.FromEuler(Rotation).Slerp(rotation, _rotationSpeed * (float)delta).GetEuler();
		}
	}

	private void MovePlayer(double delta)
	{
		Vector3 velocity = Velocity;		
		
		// Add the gravity
		if (!IsOnFloor())
			velocity.Y -= _gravity * (float)delta;

		// Handle Jump
		if (Input.IsActionJustPressed("jump") && IsOnFloor())
			velocity.Y = _jumpVelocity;
		
		Vector2 inputDir = Input.GetVector("move_left", "move_right", "move_forward", "move_back");
		Vector3 direction = new Vector3(inputDir.X, 0, inputDir.Y).Normalized();
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