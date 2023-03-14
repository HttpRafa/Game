using System.Collections.Generic;
using Godot;

namespace Game.scripts.entities.player;

public partial class RemotePlayer : CharacterBody3D
{
	
	[Signal]
	public delegate void MouseInputStateChangedEventHandler(bool enabled);

	[Export] private float _gravity = ProjectSettings.GetSetting("physics/3d/default_gravity").AsSingle();
	[Export] private float _speed = 6.0f;
	[Export] private float _jumpVelocity = 4.5f;
	[Export] private float _rotationSpeed = 15f;

	[Export] private MeshInstance3D _modelInstance;
	[Export] private StandardMaterial3D _materials;
	[Export] private StandardMaterial3D _materials1;
	[Export] private StandardMaterial3D _materials2;

	public static int PlayerIndex = 0;

	private bool _isLocal;
	private bool _isUsingMouse = true;
	
	private Vector3 _rotationTarget;

	public override void _EnterTree()
	{
		if (PlayerIndex == 0)
		{
			_modelInstance.MaterialOverride = _materials;	
		}
		if (PlayerIndex == 1)
		{
			_modelInstance.MaterialOverride = _materials1;	
		}
		if (PlayerIndex == 2)
		{
			_modelInstance.MaterialOverride = _materials2;	
		}
		GD.Print(Name.ToString() + " is player number " + PlayerIndex);
		PlayerIndex++;
		
		SetMultiplayerAuthority(Name.ToString().ToInt());
		
		if (IsMultiplayerAuthority())
		{
			_isLocal = true;
			LocalPlayer.PlayerBody = this;
			LocalPlayer.Instance.Crosshair.RotationTargetChanged += target =>
			{
				_rotationTarget = target;
			};
		}
	}

	public override void _PhysicsProcess(double delta)
	{
		if (_isUsingMouse && (Input.IsActionPressed("look_up") || Input.IsActionPressed("look_down") || Input.IsActionPressed("look_right") || Input.IsActionPressed("look_left")))
		{
			_isUsingMouse = false;
			LocalPlayer.Instance.Crosshair.Hide();
			
			EmitSignal(SignalName.MouseInputStateChanged, false);
		} else if (!_isUsingMouse && (Input.GetLastMouseVelocity() > Vector2.Zero))
		{
			_isUsingMouse = true;
			LocalPlayer.Instance.Crosshair.Show();

			EmitSignal(SignalName.MouseInputStateChanged, true);
		}

		if (IsMultiplayerAuthority())
		{
			MovePlayer(delta);
			RotatePlayer(delta);	
		}
	}

	private void RotateWeapon()
	{
		if(_isUsingMouse)
		{
			LocalPlayer.Instance.Crosshair.GlobalTransform = LocalPlayer.Instance.Crosshair.GlobalTransform.LookingAt(_rotationTarget, Vector3.Up);
		}
		else
		{
			Rotation = new Vector3(0f, 0f, 0f);
		}
	}
	
	private void RotatePlayer(double delta)
	{
		if (_isUsingMouse)
		{
			if (_rotationTarget != Vector3.Zero)
			{
				Vector3 target = _rotationTarget;
				target.Y = Position.Y;
				Quaternion rotation = Transform.LookingAt(target, Vector3.Up).Basis.GetRotationQuaternion();
				Rotation = Quaternion.FromEuler(Rotation).Slerp(rotation, _rotationSpeed * (float)delta).GetEuler();	
			}
		}
		else
		{
			Vector2 gamepadInput = Input.GetVector("look_left", "look_right", "look_up", "look_down");
			if (gamepadInput != Vector2.Zero)
			{
				Vector3 gamepadLook = new Vector3(gamepadInput.X, 0, gamepadInput.Y) + Position;
				Quaternion rotation = Transform.LookingAt(gamepadLook, Vector3.Up).Basis.GetRotationQuaternion();
				Rotation = Quaternion.FromEuler(Rotation).Slerp(rotation, _rotationSpeed * (float)delta).GetEuler();	
			}
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