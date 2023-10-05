import Types "types";
import Utils "utils";
import HashMap "mo:base/HashMap";
import Text "mo:base/Text";
import List "mo:base/List";
import Result "mo:base/Result";
import Iter "mo:base/Iter";
import Debug "mo:base/Debug";

actor {

  // ------------------------------------------------------------------Direct Chat Service------------------------------------------------------------------  

  type User = Types.User;
  type DirectMessage = Types.DirectMessage;
  
  //USERS NOT RELEVANT FOR NOW
  // Map of the users in the chat
  var users = HashMap.HashMap<Text, User>(0, Text.equal, Text.hash);

  // Map of the direct messages chats, with the chat id (username-username) as the key and a map of the messages as the value
  var chats = HashMap.HashMap<Text, HashMap.HashMap<Text, DirectMessage>>(0, Text.equal, Text.hash);

  // Map of conversations for a users, with the user name as the key and a lists of chats id's that corresponds to the actual chats map
  var userChats = HashMap.HashMap<Text, List.List<Text>>(0, Text.equal, Text.hash);

  public shared func createUser(args : User) : () {
    users.put(args.userName, args);
  };

  public shared func deleteUser(name : Text) : () {
    users.delete(name);
  };

  public shared func updateUser(args : User) : async Result.Result<(), Text> {
    switch (users.get(args.userName)) {
      case (null) { return #err("No user found with the given username") };
      case (?user) {
        ignore users.replace(user.userName, args);
        return #ok();
      };
    };
  };

  public shared query func getUser(name : Text) : async Result.Result<User, Text> {
    switch (users.get(name)) {
      case (null) { return #err("No user found with the given username") };
      case (?user) { return #ok(user) };
    };
  };

  public shared query func getUsers() : async [User] {
    return Iter.toArray(users.vals());
  };

  public shared func sendMessage(args : DirectMessage) : async Result.Result<(), Text> {

    // Check if the chat already exists
    switch (chats.get(args.chatId)) {
      // If it does not exist, create a new chat, add the message to it and add the chat to the two users chat lists
      case (null) {
        Debug.print("no chat exist, so we are creating one" # debug_show (args.chatId));
        let chatId = Utils.generate_chat_id(args.sender, args.receiver);
        var newChat = HashMap.HashMap<Text, DirectMessage>(0, Text.equal, Text.hash);
        newChat.put(args.messageId, args);
        Debug.print("chat id here" # debug_show (chatId));
        chats.put(chatId, newChat);
        var sendingUserChats : List.List<Text> = switch (userChats.get(args.sender)) {
          case (null) {
            List.nil<Text>();
          };
          case (?chatList) {
            chatList;
          };
        };
        sendingUserChats := List.push(chatId, sendingUserChats);
        userChats.put(args.sender, sendingUserChats);
        
        var receivingUserChats : List.List<Text> = switch (userChats.get(args.receiver)) {
          case (null) { List.nil<Text>() };
          case (?chatList) { chatList };
        };
        receivingUserChats := List.push(chatId, receivingUserChats);
        userChats.put(args.receiver, receivingUserChats);
        return #ok();
      };
      // If it does exist, add the message to it
      case (?chat) {
        Debug.print("chat exist, so we are adding to it" # debug_show (args.chatId));
        let messageId = Utils.generate_uuid();
        chat.put(messageId, args);
        return #ok();
      };
    };
  };

  public shared query func getMessages(chatId : Text) : async Result.Result<[DirectMessage], Text> {
    switch (chats.get(chatId)) {
      case (null) { return #err("No chat found with the given chat id") };
      case (?chat) {
        return #ok(Iter.toArray(chat.vals()));
      };
    };
  };

  public shared query func getUserChats(userName : Text) : async [Text] {
    var chatList : List.List<Text> = switch (userChats.get(userName)) {
      case (null) {
        List.nil<Text>();
      };
      case (?chatList) {
        chatList;
      };
    };
    return List.toArray(chatList);
  };

  public shared func deleteChat(chatId : Text) : async Result.Result<(), Text> {
    switch (chats.get(chatId)) {
      case (null) { return #err("No chat found with the given chat id") };
      case (?chat) {
        chats.delete(chatId);
        return #ok();
      };
    };
  };

  public shared func deleteMessage(chatId : Text, messageId : Text) : async Result.Result<(), Text> {
    switch (chats.get(chatId)) {
      case (null) { return #err("No chat found with the given chat id") };
      case (?chat) {
        switch (chat.get(messageId)) {
          case (null) {
            return #err("No message found with the given message id");
          };
          case (?message) {
            chat.delete(messageId);
            return #ok();
          };
        };
      };
    };
  };

  public shared func editMessage(args : DirectMessage) : async Result.Result<(), Text> {
    switch (chats.get(args.chatId)) {
      case (null) { return #err("No chat found with the given chat id") };
      case (?chat) {
        switch (chat.get(args.messageId)) {
          case (null) {
            return #err("No message found with the given message id");
          };
          case (?message) {
            ignore chat.replace(message.messageId, args);
            return #ok();
          };
        };
      };
    };
  };

  // ------------------------------------------------------------------Group Chat Service------------------------------------------------------------------

  type Group = Types.Group;
  type GroupMessage = Types.GroupMessage;

  // Map of the groups in the chat
  var groups = HashMap.HashMap<Text, Group>(0, Text.equal, Text.hash);
  var groupChats = HashMap.HashMap<Text, HashMap.HashMap<Text, GroupMessage>>(0, Text.equal, Text.hash);

  public shared func createGroup(args : Group) : () {
    groups.put(args.id, args);
  };

  public shared func deleteGroup(id : Text) : () {
    groups.delete(id);
  };

  public shared func updateGroup(args : Group) : async Result.Result<(), Text> {
    switch (groups.get(args.id)) {
      case (null) { return #err("No group found with the given id") };
      case (?group) {
        ignore groups.replace(group.id, args);
        return #ok();
      };
    };
  };

  public shared query func getGroup(id : Text) : async Result.Result<Group, Text> {
    switch (groups.get(id)) {
      case (null) { return #err("No group found with the given id") };
      case (?group) { return #ok(group) };
    };
  };

  public shared query func getGroups() : async [Group] {
    return Iter.toArray(groups.vals());
  };

  public shared func sendGroupMessage(args : GroupMessage) : async Result.Result<(), Text> {
    switch (groupChats.get(args.id)) {
      case (null) {
        let newGroupChat = HashMap.HashMap<Text, GroupMessage>(0, Text.equal, Text.hash);
        newGroupChat.put(args.id, args);
        groupChats.put(args.id, newGroupChat);
        return #ok();
      };
      case (?groupChat) {
        let messageId = Utils.generate_uuid();
        groupChat.put(messageId, args);
        return #ok();
      };
    };
  };

  public shared query func getGroupMessages(id : Text) : async Result.Result<[GroupMessage], Text> {
    switch (groupChats.get(id)) {
      case (null) { return #err("No group chat found with the given id") };
      case (?groupChat) {
        return #ok(Iter.toArray(groupChat.vals()));
      };
    };
  };

  public shared func deleteGroupMessage(id : Text, messageId : Text) : async Result.Result<(), Text> {
    switch (groupChats.get(id)) {
      case (null) { return #err("No group chat found with the given id") };
      case (?groupChat) {
        switch (groupChat.get(messageId)) {
          case (null) {
            return #err("No message found with the given message id");
          };
          case (?message) {
            groupChat.delete(messageId);
            return #ok();
          };
        };
      };
    };
  };

  public shared func editGroupMessage(args : GroupMessage) : async Result.Result<(), Text> {
    switch (groupChats.get(args.id)) {
      case (null) { return #err("No group chat found with the given id") };
      case (?groupChat) {
        switch (groupChat.get(args.id)) {
          case (null) {
            return #err("No message found with the given message id");
          };
          case (?message) {
            ignore groupChat.replace(message.id, args);
            return #ok();
          };
        };
      };
    };
  };
  
  
    

};
