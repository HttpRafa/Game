using Godot;

namespace Game.scripts.entities.player;

public partial class PlayerMovement : CharacterBody3D
{
	[Export] private float _speed = 5.0f;
	[Export] private float _jumpVelocity = 4.5f;

	// Get the gravity from the project settings to be synced with RigidBody nodes.
	private readonly float _gravity = ProjectSettings.GetSetting("physics/3d/default_gravity").AsSingle();

	public override void _PhysicsProcess(double delta)
	{
		Vector3 velocity = Velocity;

		// Add the gravity
		if (!IsOnFloor())
			velocity.Y -= _gravity * (float)delta;

		/* Handle Jump
		if (Input.IsActionJustPressed("ui_accept") && IsOnFloor())
			velocity.Y = _jumpVelocity;
		*/

		// Get the input direction and handle the movement/deceleration
		// As good practice, you should replace UI actions with custom gameplay actions
		Vector2 inputDir = Input.GetVector("ui_left", "ui_right", "ui_up", "ui_down");
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