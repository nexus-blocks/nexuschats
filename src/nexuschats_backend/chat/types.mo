import Text "mo:base/Text";
import Bool "mo:base/Bool";

module {
    public type User = {
        id : Text;
        userName : Text;
        email : ?Text;
        profilePicture : ?Text;
        status : ?Text;
        lastSeen : ?Int;
        dateJoined : Int;

    };
    public type DirectMessage = {
        sender : Text;
        receiver : Text;
        body : MessageBody;
        messageId : Text;
        created : Int;
        edited : Bool;
        chatId : Text;
    };

    public type Group = {
        id : Text;
        name : Text;
        description : ?Text;
        profilePicture : ?Text;
        creator : GroupCreator;
        members : [Text];
        admins : [Text];
        isPrivate : Bool;
        created : Int;
        messages : ?Text;
        lastMessage : ?Text;
        lastMessageTime : ?Int;
    };

    public type GroupCreator = {
        userName : Text;
        profilePicture : ?Text;
    };

    public type GroupMessage = {
        id : Text;
        sender : Text;
        body : MessageBody;
        created : Int;
        edited : Bool;
    };

    type MessageBody = {
        text : ?Text;
        image : ?Text;
        video : ?Text;
    };

    public type Chunk_ID = Nat;

    public type ChunkInfo = { id : Chunk_ID; order : Nat };

};
