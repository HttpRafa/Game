using System;
using Game.scripts.entities.player;
using Godot;

namespace Game.scripts;

public partial class GameWorld : Node3D
{

	[Export] private Node3D _networkNode;
	[Export] private PackedScene _remotePlayerScene;

	private ENetMultiplayerPeer _multiplayerPeer = new();
	
	public override void _Ready()
	{
	}

	private void AddNetworkPlayer(long peerId)
	{
		Node remotePlayer = _remotePlayerScene.Instantiate();
		remotePlayer.Name = new StringName(peerId.ToString());
		_networkNode.AddChild(remotePlayer);
	}
	
	private void NetworkInit()
	{
		Multiplayer.MultiplayerPeer = _multiplayerPeer;
		Multiplayer.PeerConnected += id =>
		{
			GD.Print("Connected " + id + "...");
			AddNetworkPlayer(id);
		};
	}

	public void StartHost(int port)
	{
		GD.Print("Starting host on port " + port + "...");

		_multiplayerPeer.CreateServer(port);
		NetworkInit();
		AddNetworkPlayer(Multiplayer.GetUniqueId());
	}

	public void StartServer(int port)
	{
		GD.Print("Starting server on port " + port + "...");
		
		_multiplayerPeer.CreateServer(port);
		NetworkInit();
	}

	public void StartClient(string host, int port)
	{
		GD.Print("Starting client to " + host + ":" + port + "...");

		_multiplayerPeer.CreateClient(host, port);
		NetworkInit();
	}

}