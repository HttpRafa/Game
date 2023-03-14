using System;
using Godot;

namespace Game.scripts.menu.debug;

public partial class DebugMenu : PanelContainer
{

	[Export] private World _world;
	
	[Export] private Button _hostButton;
	[Export] private Button _joinButton;

	[Export] private LineEdit _addressEntry;

	public override void _Ready()
	{
		_hostButton.Pressed += () =>
		{
			string[] data = _addressEntry.Text.Split(":");
			if (data.Length > 1)
			{
				int port = Convert.ToInt32(data[1]);
				_world.StartHost(port);	
				Hide();
			}
		};
		_joinButton.Pressed += () =>
		{
			string[] data = _addressEntry.Text.Split(":");
			if (data.Length > 1)
			{
				int port = Convert.ToInt32(data[1]);
				_world.StartClient(data[0], port);		
				Hide();
			}
		};
	}

}