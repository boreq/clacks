export enum ShutterLocation {
    TopLeft = 'TOP_LEFT',
    TopRight = 'TOP_RIGHT',
    MiddleLeft = 'MIDDLE_LEFT',
    MiddleRight = 'MIDDLE_RIGHT',
    BottomLeft = 'BOTTOM_LEFT',
    BottomRight = 'BOTTOM_RIGHT',
}

export enum ShutterPosition {
    Open = 'OPEN',
    Closed = 'CLOSED',
}

export interface CurrentMessage {
    before: MessagePart[];
    current?: MessagePart;
    after: MessagePart[];
}

export interface Message {
    parts: MessagePart[];
}

export interface MessagePart {
    kind: MessagePartKind
    character?: string,
    openShutters: ShutterLocation[],
}

export enum MessagePartKind {
    Character = 'CHARACTER',
    End = 'END',
}

export interface DisplayedError {
    message: string;
    createdAt: Date;
}
