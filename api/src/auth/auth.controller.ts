import { Controller, Post, Body, Get, UseGuards, Request } from '@nestjs/common';
import { AuthService } from './auth.service';
import { AuthGuard } from '@nestjs/passport';

@Controller('auth')
export class AuthController {
  constructor(private readonly authService: AuthService) {}

  /** GET /auth/challenge?account=G... — returns a SEP-10 challenge transaction */
  @Get('challenge')
  async getChallenge(@Request() req): Promise<{ transaction: string; network_passphrase: string }> {
    const account = req.query.account as string;
    return this.authService.generateChallenge(account);
  }

  /** POST /auth/token — verifies signed challenge, returns JWT */
  @Post('token')
  async getToken(@Body() body: { transaction: string }): Promise<{ access_token: string }> {
    return this.authService.verifyAndIssueToken(body.transaction);
  }

  /** GET /auth/me — protected route, returns authenticated account */
  @UseGuards(AuthGuard('jwt'))
  @Get('me')
  getMe(@Request() req): { account: string } {
    return { account: req.user.account };
  }
}
